use polodb_core::Database;
use polodb_core::bson::doc;

// Define a struct to represent your data (VG table rows)
#[derive(Debug)]
struct VGRow {
    // Define your fields here
    field1: String,
    field2: i32,
}

impl VGRow {
    // Constructor method for VGRow
    fn new(field1: &str, field2: i32) -> Self {
        VGRow {
            field1: field1.to_string(),
            field2,
        }
    }
}

fn create_and_populate_database() {
    // Create or open the database
    let mut db = Database::open_file("vgdata.db").unwrap();

    let test_collection = db.collection("test");

    




    // Create VG table
    let mut vg_table = database
        .create_table::<VGRow>("VG")
        .expect("Failed to create VG table");

    // Insert rows into VG table
    let row1 = VGRow::new("Value1", 42);
    let row2 = VGRow::new("Value2", 73);

    vg_table.insert(&row1).expect("Failed to insert row1");
    vg_table.insert(&row2).expect("Failed to insert row2");

    // Print rows from VG table
    println!("VG Table Rows:");

    for (key, value) in vg_table.iter() {
        println!("{:?}: {:?}", key, value);
    }
}

fn main() {
    create_and_populate_database();
}
use polodb::{Database, Collection, Value};

fn main() {
    // Create a database instance
    let mut db = Database::new("my_database.db");

    // Create a collection named "users"
    let users_collection = db.create_collection("users");

    // Define some sample records
    let records = vec![
        Value::Object(vec![
            ("name".to_string(), Value::String("Alice".to_string())),
            ("age".to_string(), Value::Number(30.0)),
        ]),
        Value::Object(vec![
            ("name".to_string(), Value::String("Bob".to_string())),
            ("age".to_string(), Value::Number(25.0)),
        ]),
    ];

    // Insert records into the collection
    for record in records {
        users_collection.insert(record)?;
    }

    // Print the contents of the collection
    for doc in users_collection.iter()? {
        println!("{:?}", doc);
    }

    // Close the database connection
    db.close()?;
}
