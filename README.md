# Project Name

## Description

This little script is designed to update a application users timegate pin from a CSV file. The CSV file should contain the following columns:
First Name, Last Name, Some columns, Timegate Pin (as the 8th column)

## Installation

To use this project, follow these steps:

1. Clone the repository.
2. Create a `.env` file in the root directory.
3. Add the following environment variables to the `.env` file:

```
MONGO_URI=[your MongoDB connection string]
CSV_FILE=[path to your CSV file]
```

## Usage

 Run the command `cargo run [your application ID]` to start the application.

