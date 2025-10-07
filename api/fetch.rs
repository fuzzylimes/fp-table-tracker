use http::{Method, StatusCode};
use mongodb::{bson::doc, Client};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    time::{SystemTime, UNIX_EPOCH},
};
use vercel_runtime::{run, Body, Error, Request, Response};

// Struct for the API response
#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Vec<PokerTable>,
}

// Struct for individual poker table data
#[derive(Debug, Deserialize)]
struct PokerTable {
    name: String,
    limit: String,
    #[serde(rename = "numberOfGames")]
    number_of_games: u8,
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

async fn handler(request: Request) -> Result<Response<Body>, Error> {
    let key = env::var("SECRET_KEY").unwrap();
    
    if request.method() != Method::PUT {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Bad Request".into())?);
    }

    if !request.headers().contains_key("auth") || request.headers().get("auth").unwrap().ne(&key) {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Bad Request".into())?);
    }

    let api_response = query().await?;
    let res = parse(&api_response);
    println!("{:?}", res);
    write_to_mongo(&res).await?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body("".into())?)
}

async fn query() -> Result<ApiResponse, Error> {
    let payload = serde_json::json!({
        "instance_id": "poker-tables-295-68e4eee4cd552",
        "paragraph_id": "295"
    });

    let client = reqwest::Client::new();
    let body = client
        .post("https://foxwoods.com/api/poker-tables/load")
        .header("Host", "foxwoods.com")
        .header(
            "User-Agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:96.0) Gecko/20100101 Firefox/96.0",
        )
        .header("Accept", "*/*")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Accept-Encoding", "gzip, deflate, br, zstd")
        .header("X-Requested-With", "XMLHttpRequest")
        .header("Referer", "https://www.foxwoods.com/")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?
        .text()
        .await?;

    let api_response: ApiResponse = serde_json::from_str(&body)?;
    Ok(api_response)
}

fn parse(api_response: &ApiResponse) -> Record {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let ts = (ts / 60 / 30) * 60 * 30; // get nearest 30 minutes

    let mut games: HashMap<String, Vec<Game>> = HashMap::new();
    let mut table_count: u8 = 0;

    // Process the data array directly
    for table in &api_response.data {
        let game = Game {
            table_count: table.number_of_games,
            blinds: table.limit.clone(),
        };
        table_count += game.table_count;

        // Clean up the game name (remove extra spaces)
        let key = table.name.replace("   ", " ");

        if let Some(game_list) = games.get_mut(&key) {
            game_list.push(game);
        } else {
            games.insert(key, vec![game]);
        }
    }

    Record {
        ts,
        table_count,
        games,
    }
}

async fn write_to_mongo(record: &Record) -> Result<(), Error> {
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
    let client = Client::with_uri_str(client_string).await?;
    let database = client.database("foxwoods");
    let collection = database.collection::<Record>(&mongo_collection);

    collection.insert_one(record).await?;
    
    Ok(())
}
