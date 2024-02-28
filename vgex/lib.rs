use polodb_core::{Database,Collection};
use polodb_core::bson::{Document, doc};
use serde::{Deserialize, Serialize};

use csv::ReaderBuilder;
use reqwest::blocking::get;
// use serde_json::{Value, Map};

// fn csv_to_dicts(csv_url: &str) -> Result<Vec<serde_json::Map<String, serde_json::Value>>, reqwest::Error> {
//     // Fetch CSV data from the provided URL
//     let response = get(csv_url)?;
//     let csv_data = response.text()?;

//     // Parse CSV data using csv crate
//     let mut reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());
//     let headers = reader.headers().unwrap().clone();

//     // Convert CSV rows to dictionaries
//     let result: Result<Vec<_>, _> = reader.records()
//         .map(|record| {
//             record.map(|r| {
//                 headers.iter()
//                     .map(|header| (header.to_string(), serde_json::Value::String(r[headers.iter().position(|h| h == header).unwrap()].to_string())))
//                     .collect::<serde_json::Map<_, _>>()
//             })
//         })
//         .collect();

//     Ok(result.unwrap())
// }


fn csv_to_game_records(csv_url: &str) -> Result<Vec<GameRecord>, reqwest::Error> {
    // Fetch CSV data from the provided URL
    let response = get(csv_url)?;
    let csv_data = response.text()?;

    // Parse CSV data using csv crate
    let mut reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());
    let headers = reader.headers().unwrap().clone();

    // Convert CSV rows to GameRecord instances
    let result: Result<Vec<_>, _> = reader.records()
        .map(|record| {
            record.map(|r| {
                GameRecord {
                    rank: r[headers.iter().position(|h| h == "Rank").unwrap()].to_string(),
                    name: r[headers.iter().position(|h| h == "Name").unwrap()].to_string(),
                    platform: r[headers.iter().position(|h| h == "Platform").unwrap()].to_string(),
                    year: r[headers.iter().position(|h| h == "Year").unwrap()].to_string(),
                    genre: r[headers.iter().position(|h| h == "Genre").unwrap()].to_string(),
                    publisher: r[headers.iter().position(|h| h == "Publisher").unwrap()].to_string(),
                    na_sales: r[headers.iter().position(|h| h == "NA_Sales").unwrap()].to_string(),
                    eu_sales: r[headers.iter().position(|h| h == "EU_Sales").unwrap()].to_string(),
                    jp_sales: r[headers.iter().position(|h| h == "JP_Sales").unwrap()].to_string(),
                    other_sales: r[headers.iter().position(|h| h == "Other_Sales").unwrap()].to_string(),
                    global_sales: r[headers.iter().position(|h| h == "Global_Sales").unwrap()].to_string(),
                }
            })
        })
        .collect();

    Ok(result.unwrap())
}



// let keys = doc! {
//     "user_id": 1,
//   };

// #[derive(Debug, Serialize, Deserialize)]
// struct Game {
//     title: String,
//     global_sales: f32,
// }

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GameRecord {
    rank: String,
    name: String,
    platform: String,
    year: String,
    genre: String,
    publisher: String,
    na_sales: String,
    eu_sales: String,
    jp_sales: String,
    other_sales: String,
    global_sales: String,
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
    let x = csv_to_game_records("https://vgexo.s3.us-west-1.amazonaws.com/vgsales.csv");

    // println!("{:?}", x);

    let game_collection: Collection<GameRecord> = db.collection("games");

    game_collection.insert_many(x.unwrap()).unwrap();
    // // game_collection.insert_many(x.unwrap()).unwrap();

    print!("\n\n\n\n\n\n game_collection: \n\n\n\n\n\n\n");
    println!("{:?}", game_collection.count_documents());
    // let game_search = game_collection.find(GameRecord! {
    //     "name": " Mario ",
    // }).unwrap();
    
    // for g in game_search {
    //     println!("name: {:?}", g);
    // }

}
