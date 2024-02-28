use polodb_core::{Database,Collection};
use polodb_core::bson::{Document, doc};
use serde::{Deserialize, Serialize};

use csv::ReaderBuilder;
use reqwest::blocking::get;
use serde_json::{Value, Map};

fn csv_to_dicts(csv_url: &str) -> Result<Vec<serde_json::Map<String, serde_json::Value>>, reqwest::Error> {
    // Fetch CSV data from the provided URL
    let response = get(csv_url)?;
    let csv_data = response.text()?;

    // Parse CSV data using csv crate
    let mut reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());
    let headers = reader.headers().unwrap().clone();

    // Convert CSV rows to dictionaries
    let result: Result<Vec<_>, _> = reader.records()
        .map(|record| {
            record.map(|r| {
                headers.iter()
                    .map(|header| (header.to_string(), serde_json::Value::String(r[headers.iter().position(|h| h == header).unwrap()].to_string())))
                    .collect::<serde_json::Map<_, _>>()
            })
        })
        .collect();

    Ok(result.unwrap())
}



#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}


fn main() {
    // Create a database instance
    let  db = Database::open_memory().unwrap();

    // Create a collection named "users"
   // Get a handle to a collection of `Book`.
    let typed_collection = db.collection::<Document>("books");
    
    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];
    
    // Insert some documents into the "mydb.books" collection.
    let _ = typed_collection.insert_many(docs);

    let books = typed_collection.find(doc! {
        "author": "George Orwell",
    }).unwrap();
     
    for book in books {
        println!("name: {:?}", book);
    }

    // let x: Result<Vec<_>, reqwest::Error> = csv_to_dicts("https://vgexo.s3.us-west-1.amazonaws.com/vgsales.csv");
    let _x = csv_to_dicts("https://vgexo.s3.us-west-1.amazonaws.com/vgsales.csv");

    // println!("{:?}", x);

    // let _game_collection: Collection<Vec<Map<String,Value>>> = db.collection("games");

    // game_collection.insert_many(x.unwrap()).unwrap();

}
