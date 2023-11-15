use std::process::Command;
use rocket::serde::json::Json;
use crate::models::status_model::CustomStatus;

#[get("/search/<upc>")]
pub fn lookup(upc: &str) -> Result<String, Json<CustomStatus>>{
    let url = format!("https://api.upcitemdb.com/prod/trial/lookup?upc={}", upc);
    let output = Command::new("curl")
        .arg(url)
        .output()
        .expect("Fail");

    if output.status.success() {
        // Print the response body
        let response_body = String::from_utf8_lossy(&output.stdout);
        Ok(response_body);
    } else {
        // Print an error message if the command was not successful
        eprintln!("Curl command failed with exit code: {:?}", output.status);
    }
}