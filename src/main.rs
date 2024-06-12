use anyhow::Result;
use dotenv::dotenv;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
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
    for line in csv_lines {
        let mut user = line.split(",");
        let (first_name, last_name, pin) = match (user.next(), user.next(), user.nth(5)) {
            (Some(first_name), Some(last_name), Some(pin)) => (first_name, last_name, pin),
            _ => continue,
        };

        println!(
            "first_name: {}, last_name: {}, pin: {}",
            first_name, last_name, pin
        );

        let filter = doc! { "application": application_id.clone(), "firstName": first_name, "lastName": last_name, "deleted": false};
        let update = doc! { "$set": { "plugins.timegate.options.EmployeePIN": pin } };

        if is_dry_run == "true" {
            would_have_been_updated.push(
                db.collection::<ApplicationUser>("applicationusers")
                    .find_one(filter.clone(), None)
                    .await?
                    .unwrap(),
            );

            continue;
        }
        let result = db
            .collection::<Document>("applicationusers")
            .update_one(filter, update, None)
            .await?;

        println!("Modified: {:?}", result.modified_count);
        println!("Result: {:?}", result.upserted_id);
    }
    for user in would_have_been_updated {
        println!(
            "Would have updated: \n first_name: {},\n last_name: {},\n pin: {}",
            user.firstName, user.lastName, user.plugins.timegate.options.EmployeePIN
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
    options: Options,
}

#[derive(Debug, Serialize, Deserialize)]
struct Options {
    EmployeePIN: String,
}
