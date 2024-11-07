use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use csv::Writer;
use serde_json::Value;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct OperationLog {
    ip_address: String,
    menu_id: Option<String>,
    params: Option<String>,
    request_body: Option<Value>,
    query: Option<Value>,
    model: Option<String>,
    workbench_id: Option<String>,
}

fn process_log(log_file_path: &str) -> Result<(), Box<dyn Error>> {
    // Open the log file
    let file = File::open(log_file_path)?;
    let reader = BufReader::new(file);

    // Create a CSV writer
    let mut wtr = Writer::from_path("operation_logs.csv")?;

    // Write the CSV header
    wtr.write_record(&["ip_address", "menu_id", "params", "request_body", "query", "model", "workbench_id"])?;

    // Define the regex pattern to match operation logs
    let re = Regex::new(r#"operationLog:\s*(\{.*\})"#)?;

    // Iterate over each line in the log file
    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = re.captures(&line) {
            let json_str = &caps[1];
            let operation_log: OperationLog = serde_json::from_str(json_str)?;

            wtr.serialize(operation_log)?;
        }
    }

    // Flush the CSV writer
    wtr.flush()?;

    Ok(())
}

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the log file path was provided
    if args.len() < 2 {
        eprintln!("Usage: split_log <log_file_path>");
        return;
    }

    let log_file_path = &args[1];

    // Call the function to process the log file
    if let Err(e) = process_log(log_file_path) {
        eprintln!("Error processing log file: {}", e);
    }
}
