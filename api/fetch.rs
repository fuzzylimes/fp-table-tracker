use http::{Method, StatusCode};
use mongodb::{bson::doc, sync::Client};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::{
    collections::HashMap,
    env,
    time::{SystemTime, UNIX_EPOCH},
};
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request, Response};

// Struct for the API response
#[derive(Debug, Deserialize)]
struct ApiResponse {
    // success: bool,
    // markup: String,
    data: Vec<PokerTable>,
    // instance_id: String,
    // paragraph_type: String,
}

// Struct for individual poker table data
#[derive(Debug, Deserialize)]
struct PokerTable {
    name: String,
    limit: String,
    #[serde(rename = "numberOfGames")]
    number_of_games: u8,
    // #[serde(rename = "pockerRoomLocationId")]
    // poker_room_location_id: String,
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

    let api_response = query().unwrap();
    let res = parse(&api_response);
    println!("{:?}", res);
    write_to_mongo(&res);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body("")
        .expect("Internal Server Error");

    Ok(response)
}

fn query() -> Result<ApiResponse, Box<dyn std::error::Error>> {
    let payload = serde_json::json!({
        "instance_id": "poker-tables-295-68e4eee4cd552",
        "paragraph_id": "295"
    });

    let body: String = ureq::post("https://foxwoods.com/api/poker-tables/load")
        .set("Host", "foxwoods.com")
        .set(
            "User-Agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:96.0) Gecko/20100101 Firefox/96.0",
        )
        .set("Accept", "*/*")
        .set("Accept-Language", "en-US,en;q=0.5")
        .set("Accept-Encoding", "gzip, deflate, br, zstd")
        .set("X-Requested-With", "XMLHttpRequest")
        .set("Referer", "https://www.foxwoods.com/")
        .set("Content-Type", "application/json")
        .send_json(&payload)?
        .into_string()?;

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
