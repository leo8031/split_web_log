use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Serialize, Deserialize, Debug)]
struct LogEntry {
    logdate: String,
    logtime: String,
    ipaddress: String,
    menuid: Option<String>,
    params: Option<String>,
    requestbody: Option<String>,
    workbenchid: Option<String>,
    operation: Option<String>,
    model: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("C:\\Users\\PY245AG\\Downloads\\targus-ae-ss_micro_app_lt_logan_20240616.log")?;
    let reader = BufReader::new(file);

    let mut csv_writer = csv::Writer::from_writer(File::create("C:\\Users\\PY245AG\\Downloads\\output.csv")?);

    csv_writer.write_record(&[
        "logdate",
        "logtime",
        "ipaddress",
        "menuid",
        "params",
        "requestbody",
        "workbenchid",
        "operation",
        "model",
    ])?;

    for line in reader.lines() {
        let line = line?;
        if let Some((datetime, rest)) = line.split_once(" - operationLog: ") {
            if let Some((logdate, logtime)) = datetime.split_once(' ') {
                let logdate = logdate.to_string();
                let logtime = logtime.to_string();

                let json: Value = serde_json::from_str(rest)?;
                let ipaddress = json["ipAddress"].as_str().unwrap_or("").to_string();
                let menuid = json["menuId"].as_str().map(|s| s.to_string());
                let params = json["params"].as_str().map(|s| s.to_string());
                let requestbody = json["requestBody"].as_object().map(|obj| serde_json::to_string(obj).unwrap());
                let workbenchid = json["workbenchId"].as_str().map(|s| s.to_string());
                let model = json["model"].as_str().map(|s| s.to_string());
                let operation = json["operation"].as_str().map(|s| s.to_string());

                let log_entry = LogEntry {
                    logdate,
                    logtime,
                    ipaddress,
                    menuid,
                    params,
                    requestbody,
                    workbenchid,
                    operation,
                    model,
                };

                csv_writer.serialize(log_entry)?;
            }
        }
    }

    csv_writer.flush()?;
    Ok(())
}
