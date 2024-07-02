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
    name: Option<String>,
}

#[derive(Serialize)]
struct Response {
    #[serde(rename = "statusCode")]
    status_code: u16,
    body: String,
    headers: serde_json::Value,
}

fn name_to_greeting(name: &str) -> String {
    format!("Hello {}", name)
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let name = event.payload.body
        .and_then(|body| serde_json::from_str::<RequestBody>(&body).ok())
        .and_then(|parsed_body| parsed_body.name)
        .unwrap_or_else(|| "Guest".to_string());

    info!("Received request with name: {}", name);
    
    let greeting = name_to_greeting(&name);
    info!("Generated greeting: {}", greeting);
    
    let response = Response {
        status_code: 200,
        body: json!({ "message": greeting }).to_string(),
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