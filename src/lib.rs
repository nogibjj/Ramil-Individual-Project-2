use csv::ReaderBuilder;
use reqwest::blocking::get;
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::Path;



pub fn extract_csv(
    url: &str,
    file_path: &str,
    directory: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Check if the directory exists, if not, create it
    if !Path::new(directory).exists() {
        create_dir_all(directory)?;
    }

    // Send a GET request to the provided URL
    let response = get(url)?;
    let mut file = File::create(format!("{}/{}", directory, file_path))?;
    file.write_all(&response.bytes()?)?;

    Ok(file_path.to_string())
}



pub fn load_csv_to_db(csv_file_path: &str, db_file_path: &str) -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file = File::open(csv_file_path)?;

    // Create a CSV reader
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Connect to the SQLite database
    let conn = Connection::open(db_file_path)?;

    // Clear the table if it already exists
    conn.execute("DELETE FROM nba_draft", [])?;

    // Create the table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS nba_draft (
            Player VARCHAR(50),
            Position VARCHAR(5),
            ID VARCHAR(100),
            Draft_Year INT,
            Projected_SPM FLOAT,
            Superstar FLOAT,
            Starter FLOAT,
            Role_Player FLOAT,
            Bust FLOAT
        )",
        [],
    )?;

    // Insert each record from the CSV into the database
    for result in rdr.records() {
        let record = result?;

        // Check if the record has the expected number of fields
        if record.len() < 9 {
            eprintln!("Skipping record, not enough fields: {:?}", record);
            continue; // Skip this record if it doesn't have enough fields
        }

        // Extract fields from the CSV record
        let player = &record[0];
        let position = &record[1];
        let id = &record[2];
        let draft_year: i32 = record[3].parse().unwrap_or(0);
        let projected_spm: f64 = record[4].parse().unwrap_or(0.0);
        let superstar: f64 = record[5].parse().unwrap_or(0.0);
        let starter: f64 = record[6].parse().unwrap_or(0.0);
        let role_player: f64 = record[7].parse().unwrap_or(0.0);
        let bust: f64 = record[8].parse().unwrap_or(0.0);

        // Insert the record into the database
        conn.execute(
            "INSERT INTO nba_draft (Player, Position, ID, Draft_Year, Projected_SPM, Superstar, Starter, Role_Player, Bust)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![player, position, id, draft_year, projected_spm, superstar, starter, role_player, bust],
        )?;
    }

    Ok(())
}



/////////////////


// Function to read records from the database
pub fn read_records(conn: &Connection) -> Result<Vec<(String, String, String, i32, f64, f64, f64, f64, f64)>> {
    let mut stmt = conn.prepare(
        "SELECT Player, Position, ID, Draft_Year, Projected_SPM, Superstar, Starter, Role_Player, Bust FROM nba_draft"
    )?;
    
    let records = stmt.query_map([], |row| {
        Ok((
            row.get(0)?, // Player
            row.get(1)?, // Position
            row.get(2)?, // ID
            row.get(3)?, // Draft Year
            row.get(4)?, // Projected SPM
            row.get(5)?, // Superstar
            row.get(6)?, // Starter
            row.get(7)?, // Role Player
            row.get(8)?  // Bust
        ))
    })?;

    records.collect()
}

// Function to update a record in the database
pub fn update_record(
    conn: &Connection,
    id: &str,
    new_player: &str,
    new_position: &str,
    new_draft_year: i32,
    new_projected_spm: f64,
) -> Result<()> {
    conn.execute(
        "UPDATE nba_draft SET Player = ?, Position = ?, Draft_Year = ?, Projected_SPM = ? WHERE ID = ?",
        params![new_player, new_position, new_draft_year, new_projected_spm, id],
    )?;
    Ok(())
}

// Function to delete a record from the database
pub fn delete_record(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM nba_draft WHERE ID = ?", params![id])?;
    println!("Row successfully deleted - {}", id);
    Ok(())
}

// Function to read a record by ID
pub fn read_record_by_id(
    conn: &Connection,
    id: &str,
) -> Result<Option<(String, String, String, i32, f64, f64, f64, f64, f64)>> {
    let mut stmt = conn.prepare(
        "SELECT Player, Position, ID, Draft_Year, Projected_SPM, Superstar, Starter, Role_Player, Bust FROM nba_draft WHERE ID = ?",
    )?;

    match stmt.query_row(params![id], |row| {
        Ok((
            row.get(0)?, // Player
            row.get(1)?, // Position
            row.get(2)?, // ID
            row.get(3)?, // Draft Year
            row.get(4)?, // Projected SPM
            row.get(5)?, // Superstar
            row.get(6)?, // Starter
            row.get(7)?, // Role Player
            row.get(8)?  // Bust
        ))
    }) {
        Ok(record) => Ok(Some(record)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(err) => Err(err.into()),
    }
}

// Function to insert a new record into the database
pub fn insert_record(
    conn: &Connection,
    player: &str,
    position: &str,
    id: &str,
    draft_year: i32,
    projected_spm: f64,
    superstar: f64,
    starter: f64,
    role_player: f64,
    bust: f64,
) -> Result<()> {
    conn.execute(
        "INSERT INTO nba_draft (Player, Position, ID, Draft_Year, Projected_SPM, Superstar, Starter, Role_Player, Bust) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![player, position, id, draft_year, projected_spm, superstar, starter, role_player, bust],
    )?;
    Ok(())
}

// Function to establish a connection to the database
pub fn connect_db(database_conn: &str) -> Result<Connection> {
    let conn = Connection::open(database_conn)?;
    Ok(conn)
}

// Function to run all operations
pub fn all() -> Result<()> {
    let url = "https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/nba-draft-2015/historical_projections.csv";
    let file_path = "nba_draft.csv";
    let directory = "data";

    let result = extract_csv(url, file_path, directory);

    let csv_file_path = "data/nba_draft.csv";
    let db_file_path = "data/nba_db.sqlite";

    // Load CSV data into the database
    let result2 = load_csv_to_db(csv_file_path, db_file_path);

    // Connect to the database to perform read, update, and delete operations
    let conn = connect_db(db_file_path)?;

    // Read all records
    let records = read_records(&conn)?;
    for record in &records {
        println!("Record: {:?}", record);
    }

    // Insert a new record
    insert_record(
        &conn,
        "New Player",
        "SG",
        "new-player",
        2023,
        0.5,
        0.1,
        0.3,
        0.4,
        0.2,
    )?;
    println!("Inserted new record.");

    // Update a record
    update_record(
        &conn,
        "new-player",
        "Updated Player",
        "PF",
        2023,
        0.6,
    )?;

    if let Some(record) = read_record_by_id(&conn, "new-player")? {
        println!("Record with ID new-player: {:?}", record);
    } else {
        println!("No record found with ID new-player.");
    }

    // Delete a record
    delete_record(&conn, "new-player")?;
    if let Some(record) = read_record_by_id(&conn, "new-player")? {
        println!("Record with ID new-player: {:?}", record);
    } else {
        println!("No record found with ID new-player.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE nba_draft (
                Player TEXT,
                Position TEXT,
                ID TEXT PRIMARY KEY,
                Draft_Year INTEGER,
                Projected_SPM REAL,
                Superstar REAL,
                Starter REAL,
                Role_Player REAL,
                Bust REAL
            )",
            [],
        )
        .unwrap();
        conn
    }

    #[test]
    fn test_insert_record() {
        let conn = setup_test_db();
        let result = insert_record(
            &conn,
            "Test Player",
            "SG",
            "test_id",
            2023,
            5.0,
            0.8,
            0.6,
            0.3,
            0.1,
        );
        assert!(result.is_ok());

        let record = read_record_by_id(&conn, "test_id").unwrap();
        assert!(record.is_some());
        let record = record.unwrap();
        assert_eq!(record.0, "Test Player");
        assert_eq!(record.1, "SG");
        assert_eq!(record.2, "test_id");
        assert_eq!(record.3, 2023);
    }

    #[test]
    fn test_read_records() {
        let conn = setup_test_db();
        insert_record(
            &conn,
            "Test Player",
            "SG",
            "test_id",
            2023,
            5.0,
            0.8,
            0.6,
            0.3,
            0.1,
        )
        .unwrap();

        let records = read_records(&conn).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].0, "Test Player");
    }

    #[test]
    fn test_update_record() {
        let conn = setup_test_db();
        insert_record(
            &conn,
            "Test Player",
            "SG",
            "test_id",
            2023,
            5.0,
            0.8,
            0.6,
            0.3,
            0.1,
        )
        .unwrap();

        let result = update_record(&conn, "test_id", "Updated Player", "PF", 2024, 6.5);
        assert!(result.is_ok());

        let updated_record = read_record_by_id(&conn, "test_id").unwrap().unwrap();
        assert_eq!(updated_record.0, "Updated Player");
        assert_eq!(updated_record.1, "PF");
        assert_eq!(updated_record.3, 2024);
        assert_eq!(updated_record.4, 6.5);
    }

    #[test]
    fn test_delete_record() {
        let conn = setup_test_db();
        insert_record(
            &conn,
            "Test Player",
            "SG",
            "test_id",
            2023,
            5.0,
            0.8,
            0.6,
            0.3,
            0.1,
        )
        .unwrap();

        let result = delete_record(&conn, "test_id");
        assert!(result.is_ok());

        let record = read_record_by_id(&conn, "test_id").unwrap();
        assert!(record.is_none());
    }
}



