
use std::io;
extern crate serde_json;

use io::BufReader;
use std::fs::File;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    user_id: u32,
    id: u32,
    title: String,
    body: String
}

fn main() -> io::Result<()> {

    let file = File::open("data.json")?;
    let reader = BufReader::new(file);
    // let mut content = "".into();

    let content = serde_json::from_reader(reader)?;

    println!("content {:?}", content);


    // file.read_to_string(&mut content)?;

    // let users = serde_json::(content)?;

    // println!("content {}", content);

    Ok(())
}
