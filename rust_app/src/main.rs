use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use log::{info, error, LevelFilter};
use serde::{Deserialize, Serialize};
use serde_json::json;
use simple_logger::SimpleLogger;

#[derive(Deserialize)]
struct Request {
    body: Option<String>,
}

#[derive(Deserialize)]
struct RequestBody {
    first_name: Option<String>,
    last_name: Option<String>,
    age: i8,
}

#[derive(Serialize)]
struct Response {
    #[serde(rename = "statusCode")]
    status_code: u16,
    body: String,
    headers: serde_json::Value,
}

fn name_to_greeting(first_name: &str, last_name: &str, age: i8) -> String {
    format!("Hello, {} {}! You are {} years old.", first_name, last_name, age)
}

async fn function_handler(mut event: LambdaEvent<Request>) -> Result<Response, Error> {
    let first_name = event.payload.body
        .as_ref()
        .and_then(|body| serde_json::from_str::<RequestBody>(&body).ok())
        .and_then(|parsed_body| parsed_body.first_name.clone())
        .unwrap_or_else(|| "Guest".to_string());
    let last_name = event.payload.body.as_mut()
        .and_then(|body| serde_json::from_str::<RequestBody>(&body).ok())
        .and_then(|parsed_body| parsed_body.last_name.clone())
        .unwrap_or_else(|| "".to_string());
    let age = event.payload.body.as_ref()
        .and_then(|body| serde_json::from_str::<RequestBody>(&body).ok())
        .and_then(|parsed_body| Some(parsed_body.age));
    info!("Received request with name: {}", first_name);
    
    let greeting = name_to_greeting(&first_name, &last_name, age.unwrap_or(0));
    info!("Generated greeting: {}", greeting);
    let adult = age >= Some(18);
    let response = Response {
        status_code: 200,
        body: json!({ "first_name": first_name, "last_name": last_name, "age": age, "adult": adult }).to_string(),
        headers: json!({
            "Content-Type": "application/json",
            "Access-Control-Allow-Origin": "*"
        }),
    };

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    
    info!("Lambda function starting up");
    
    match run(service_fn(function_handler)).await {
        Ok(_) => {
            info!("Lambda function completed successfully");
            Ok(())
        },
        Err(e) => {
            error!("Lambda function error: {:?}", e);
            Err(e)
        }
    }
}