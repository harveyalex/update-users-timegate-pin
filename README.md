# Project Name

## Description

This little script is designed to update a application users timegate pin from a CSV file. The CSV file must look like the one in the `example.csv` file. The script will read the CSV file and update the timegate pin for each user in the file.

## Installation

To use this project, follow these steps:
0. Install Rust and cargo with rustup (https://rustup.rs/).
1. Clone the repository.
2. Create a `.env` file in the root directory.
3. Add the following environment variables to the `.env` file:

```
MONGO_URI=[your MongoDB connection string]
CSV_FILE=[path to your CSV file]
DRY_MODE=[true|false]
```

## Usage

 Run the command `cargo run "<applicationId>" "<development|production>"` to start the application.

