use anyhow::Result;
use dotenv::dotenv;
use mongodb::results::UpdateResult;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let application_id = ObjectId::from_str(
        std::env::args()
            .nth(1)
            .unwrap_or("000000000000000000000000".to_string())
            .as_str(),
    )
    .unwrap();
    let database_id = std::env::args().nth(2).unwrap_or("development".to_string());
    let database_id = &database_id.as_str();
    let is_dry_run = std::env::var("DRY_RUN").unwrap_or("true".to_string());
    println!("Application ID: {:?}", application_id);
    println!("Is dry run: {:?}", is_dry_run);
    let csv_file = std::env::var("CSV_FILE")?;

    let csv_data = std::fs::read_to_string(&csv_file)?;
    let mut csv_lines: Vec<String> = csv_data.lines().map(|line| line.to_string()).collect();

    let mongo_uri = std::env::var("MONGO_URI").unwrap();
    let client = Client::with_uri_str(&mongo_uri).await?;
    let db = client.database(database_id);

    csv_lines.remove(0); // ignore the headers
    let mut would_have_been_updated: Vec<ApplicationUser> = vec![];
    let mut updated: Vec<UpdateResult> = vec![];
    for line in csv_lines {
        let mut user = line.split(",");
        let (first_name, last_name, pin) = match (user.next(), user.next(), user.nth(5)) {
            (Some(first_name), Some(last_name), Some(pin)) => (first_name, last_name, pin),
            _ => continue,
        };

        let filter = doc! { "application": application_id.clone(), "firstName": first_name, "lastName": last_name, "deleted": {"$ne": true} };
        let update = doc! { "$set": { "plugins.timegate.options.EmployeePIN": pin } };

        if is_dry_run == "true" {
            match db
                .collection::<ApplicationUser>("applicationusers")
                .find_one(filter.clone(), None)
                .await?
            {
                Some(user) => {
                    println!("User: {:?}", user);
                    would_have_been_updated.push(user);
                }
                _ => {
                    println!("User not found");
                }
            }
            continue;
        }
        let result = db
            .collection::<ApplicationUser>("applicationusers")
            .update_one(filter, update, None)
            .await?;
        println!("Updated user: {}", first_name);
        updated.push(result);
    }
    if is_dry_run == "true" {
        println!("Would have updated: {:?}", would_have_been_updated.len());
        for user in would_have_been_updated {
            println!(
                "first_name: {},\n last_name: {},\n pin: {}",
                user.firstName,
                user.lastName,
                user.plugins
                    .timegate
                    .options
                    .expect("Should have employee pin?")
                    .employeePIN
            );
        }
        return Ok(());
    }
    for result in updated {
        println!(
            "Matched: {}, Modified: {}",
            result.matched_count, result.modified_count
        );
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct ApplicationUser {
    firstName: String,
    lastName: String,
    plugins: Plugins,
}

#[derive(Debug, Serialize, Deserialize)]
struct Plugins {
    timegate: TimeGate,
}

#[derive(Debug, Serialize, Deserialize)]
struct TimeGate {
    options: Option<Options>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Options {
    employeePIN: String,
}
