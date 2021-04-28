
use std::io;
extern crate serde_json;
extern crate serde;

use io::BufReader;
use std::fs;
use serde::{Serialize, Deserialize};
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

    let file = fs::File::open("data.json")
        .expect("Error when opening data.json file");

    let reader = BufReader::new(file);


    let users: Vec<User> = serde_json::from_reader(reader)?;

    // Just removing duplication of users with the same user_id
    let updated_users: Vec<User> = update_data(users);

    // We can stringify our data using serde, but we don't need when using to_writer_pretty
    // let json = serde_json::to_string(&updated_users)?;

    let new_file = fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open("created.json")
        .expect("Error when opening created.json file.");

    // Using serde to prettify json when writing file.
    serde_json::to_writer_pretty(new_file, &updated_users)?;

    // Writing file using fs
    // let created = fs::write(new_file, &json)?;

    Ok(())
}

fn update_data(users: Vec<User>) -> Vec<User> {
    let filtered_users: Vec<User> = users
        .into_iter()
        .fold(Vec::new(), |mut acc, user| {
            let already_exist = acc.iter().any(|x| x.user_id == user.user_id);

            if already_exist {
                return acc;
            };

            acc.push(user);

            acc
        });

    filtered_users
}
