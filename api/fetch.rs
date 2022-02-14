use http::{Method, StatusCode};
use mongodb::{bson::doc, sync::Client};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::{
    collections::HashMap,
    env,
    time::{SystemTime, UNIX_EPOCH},
};
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request, Response};

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    let key = env::var("SECRET_KEY").unwrap();
    let bad_request = Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("Bad Request")
        .expect("Bad Request");

    if request.method() != Method::PUT {
        return Ok(bad_request);
    }

    if !request.headers().contains_key("auth") || request.headers().get("auth").unwrap().ne(&key) {
        return Ok(bad_request);
    }

    let html = query().unwrap();
    let res = parse(html.as_str());
    println!("{:?}", res);
    write_to_mongo(&res);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body("")
        .expect("Internal Server Error");

    Ok(response)
}

fn query() -> Result<String, ureq::Error> {
    let body: String = ureq::get("https://www.foxwoods.com/casino/choose/poker/CurrentPokerTablesBlock_GetCurrentPokerTables")
        .set("Host", " www.foxwoods.com")
        .set(
            "User-Agent",
            " Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:96.0) Gecko/20100101 Firefox/96.0",
        )
        .set("Accept", " text/html, */*; q=0.01")
        .set("Accept-Language", " en-US,en;q=0.5")
        .set("Accept-Encoding", " gzip, deflate, br")
        .set("X-Requested-With", " XMLHttpRequest")
        .set("Referer", " https://www.foxwoods.com/casino/choose/poker/")
        .call()?
        .into_string()?;
    Ok(body)
}

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    ts: u64,
    #[serde(rename = "tableCount")]
    table_count: u8,
    games: HashMap<String, Vec<Game>>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct Game {
    #[serde(rename = "tableCount")]
    table_count: u8,
    blinds: String,
}

fn parse(html: &str) -> Record {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut games: HashMap<String, Vec<Game>> = HashMap::new();
    let mut table_count: u8 = 0;

    let parsed = Html::parse_fragment(html);
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    for tr in parsed.select(&tr_selector).skip(1) {
        let tds = tr
            .select(&td_selector)
            .map(|row| row.inner_html())
            .collect::<Vec<_>>();
        let ng = Game {
            table_count: (&tds[0][1..&tds[0].len() - 1]).parse().unwrap(),
            blinds: tds[1].clone(),
        };
        table_count += ng.table_count;

        let key = tds[2].replace("   ", " ");
        if games.contains_key(&key) {
            games.get_mut(&key).unwrap().push(ng);
        } else {
            games.insert(key, vec![ng]);
        }
    }

    Record {
        ts,
        table_count,
        games,
    }
}

fn write_to_mongo(record: &Record) {
    let mongo_user = env::var("SCRAPE_USER").unwrap();
    let mongo_pass = env::var("SCRAPE_PASS").unwrap();
    let mongo_url = env::var("MONGO_URL").unwrap();
    let mongo_db = env::var("MONGO_DB").unwrap();
    let mongo_collection = env::var("MONGO_COLLECTION").unwrap();

    let client_string = format!(
        "mongodb+srv://{mongoUser}:{mongoPass}@{mongoUrl}/{mongoDb}",
        mongoUser = mongo_user,
        mongoPass = mongo_pass,
        mongoUrl = mongo_url,
        mongoDb = mongo_db
    );
    let client = Client::with_uri_str(client_string).expect("Error creating mongo client");
    let database = client.database("foxwoods");
    let collection = database.collection::<Record>(&mongo_collection);

    collection
        .insert_one(record, None)
        .expect("Error writing data to mongo");
}
