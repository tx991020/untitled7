use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;
use anyhow::Result;
use chrono::Utc;
use mongodb::bson::{doc, Document};
use mongodb::bson::oid::ObjectId;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    a: String,
    b: String,

}




#[tokio::main]
async fn main() -> Result<()> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri = String::from("mongodb://localhost:27017/?readPreference=primary&appname=MongoDB%20Compass%20Beta&directConnection=true&ssl=false");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    // Print the databases in our MongoDB cluster:
    let movies = client.database("test").collection("coll");

    // Look up one document:
    let movie: Movie = movies
        .find_one(
            doc! {},
            None,
        ).await?
        .expect("Missing 'Parasite' document.");
    println!("Movie: {:?}", movie.a);


    Ok(())
}