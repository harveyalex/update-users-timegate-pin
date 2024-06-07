use anyhow::Result;
use dotenv::dotenv;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Client,
};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let application_id = ObjectId::from_str(std::env::args().nth(1).unwrap().as_str()).unwrap();
    println!("Application ID: {:?}", application_id);
    let csv_file = std::env::var("CSV_FILE")?;

    let csv_data = std::fs::read_to_string(&csv_file)?;
    let mut csv_lines: Vec<String> = csv_data.lines().map(|line| line.to_string()).collect();

    let mongo_uri = std::env::var("MONGO_URI").unwrap();
    let client = Client::with_uri_str(&mongo_uri).await?;
    let db = client.database("development");

    csv_lines.remove(0); // ignore the headers
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

        let filter = doc! { "application": application_id.clone(), "firstName": first_name, "lastName": last_name };
        let update = doc! { "$set": { "plugins.timegate.options.employeePIN": pin } };

        let result = db
            .collection::<Document>("applicationusers")
            .update_one(filter, update, None)
            .await?;

        println!("Modified: {:?}", result.modified_count);
        println!("Result: {:?}", result.upserted_id);
    }

    Ok(())
}
