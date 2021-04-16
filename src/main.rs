
use std::io;
extern crate serde_json;
extern crate serde;

use io::BufReader;
use std::fs::File;
use serde::{Deserialize, Serialize};
// use serde_json::Value as JsonValue;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    // #[serde(rename(deserialize = "user_id"))]
    user_id: u32,
    id: u32,
    title: String,
    body: String
}

fn main() -> io::Result<()> {

    let file = File::open("data.json")?;
    let reader = BufReader::new(file);

    let content: Vec<User> = serde_json::from_reader(reader)?;

    println!("content {:?}", content);

    Ok(())
}
