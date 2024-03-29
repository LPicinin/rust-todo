use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;


pub fn establish_connection() -> SqliteConnection {
    let database_url = "todoDB.sqlite";

    // Establish a connection to the database
    SqliteConnection::establish(database_url)
        .expect("Failed to connect to database")
}