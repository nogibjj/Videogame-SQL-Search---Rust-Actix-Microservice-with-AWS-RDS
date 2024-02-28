use polodb_core::{Database, Collection};
use polodb_core::bson::{Document, doc};
use std::error::Error;
// use std::fs::File;
// use std::io::prelude::*;
use csv::ReaderBuilder;

fn find_game_for_user(collection: &Collection<Document>, game_name: &str) {
    // Query to find a document with the specified game_name
    let query = doc! {
        "name": game_name,
    };

    // Find the document in the collection
    if let Some(result) = collection.find_one(query).unwrap() {
        println!("Results found for user: {:?}\n\n", game_name);
        println!("Game found: {:?}", result);
    } else {
        println!("No game found for user.");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create a database instance
    let db = Database::open_memory()?;

    // Create a collection named "games"
    let typed_collection = db.collection::<Document>("games");

    // Read CSV file from a URL (you might need to use a library like reqwest to fetch the CSV)
    let csv_url = "https://vgexo.s3.us-west-1.amazonaws.com/vgsales.csv";
    let response = reqwest::blocking::get(csv_url)?;
    let csv_data = response.text()?;

    // Create a CSV reader
    let mut csv_reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());

    // Iterate over each CSV record and insert it into the collection
    for result in csv_reader.records() {
        let record = result?;

        // Assuming your CSV has columns "title" and "author", adapt this to match your CSV structure
        let doc = doc! {
            "rank": &record[0], // Adjust the index based on your CSV structure
            "name": &record[1], // Adjust the index based on your CSV structure
            "platform": &record[2], // Adjust the index based on your CSV structure
            "year": &record[3], // Adjust the index based on your CSV structure
            "genre": &record[4], // Adjust the index based on your CSV structure
            "publisher": &record[5], // Adjust the index based on your CSV structure
            "na_sales": &record[6], // Adjust the index based on your CSV structure
            "eu_sales": &record[7], // Adjust the index based on your CSV structure
            "jp_sales": &record[8], // Adjust the index based on your CSV structure
            "other_sales": &record[9], // Adjust the index based on your CSV structure
            "global_sales": &record[10], // Adjust the index based on your CSV structure
        };

        // Insert the document into the collection
        typed_collection.insert_one(doc)?;
    }

    // Example query to find and print all documents in the collection
    // let games = typed_collection.find(doc! {}).unwrap();

    // for game_record in games {
    //     println!("Game: {:?}", game_record);
    // }

    let game_search = typed_collection.find(doc! {
        "name": "Pokemon Gold/Pokemon Silver"
    })?;
    println!("\n\n\n");
    println!("Pokemon Gold/Pokemon Silver");
    for game in game_search {
        println!("\nGame: {:?}", game);
    }

    find_game_for_user(&typed_collection, "Final Fantasy VII");

    Ok(())
}
