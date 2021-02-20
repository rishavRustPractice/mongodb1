use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::BufReader;
use mongodb::{Client, Database};
use tokio::runtime::Runtime;

fn get_mongodb_uri() -> Result<String, Error> {
    let file = File::open("manifest/mongodb_uri.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut mongodb_uri = String::new();
    buf_reader.read_to_string(&mut mongodb_uri)?;
    Ok(mongodb_uri)
}

async fn connect_db() -> Result<Database, Error> 
{   
    let mongodb_uri = get_mongodb_uri()?;
    print!("{}", mongodb_uri);
    let client = Client::with_uri_str(&mongodb_uri).await.unwrap();
    let database = client.database("mydb");
    println!("Connected to {:?}", database);
    Ok(database)
}

fn main() {
    let db = connect_db();
    let db = Runtime::new().unwrap().block_on(db); //Tokio for timer, else thread panics
}
