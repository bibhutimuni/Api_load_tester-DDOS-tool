use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::time::Instant;
use chrono::{DateTime, Utc};
use reqwest::Client;
use futures::stream::StreamExt;
use csv::Reader;
use std::env;

#[derive(Debug, Deserialize)]
struct ApiRequest {
    endpoint: String,
    method: String,
    body: String,
    repetition: usize,
}

#[derive(Debug, Serialize)]
struct ApiResult {
    endpoint: String,
    request_time: DateTime<Utc>,
    response_time: DateTime<Utc>,
    method: String,
    time_taken: f64, // in seconds
    status: u16,
}

async fn send_request(
    client: &Client,
    endpoint: &str,
    method: &str,
    body: &str,
) -> Result<ApiResult, Box<dyn Error + Send + Sync>> {
    let request_time = Utc::now();
    let start = Instant::now();
    
    let response = match method.to_uppercase().as_str() {
        "GET" => client.get(endpoint).send().await?,
        "POST" => client.post(endpoint).body(body.to_string()).send().await?,
        "PUT" => client.put(endpoint).body(body.to_string()).send().await?,
        "DELETE" => client.delete(endpoint).send().await?,
        _ => return Err(format!("Unsupported HTTP method: {}", method).into()),
    };
    
    let response_time = Utc::now();
    let time_taken = start.elapsed().as_secs_f64();
    let status = response.status().as_u16();
    
    Ok(ApiResult {
        endpoint: endpoint.to_string(),
        request_time,
        response_time,
        method: method.to_string(),
        time_taken,
        status,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input.csv> <output.csv>", args[0]);
        std::process::exit(1);
    }
    
    let input_file = &args[1];
    let output_file = &args[2];
    
    let client = Client::builder()
        .pool_idle_timeout(std::time::Duration::from_secs(30))
        .pool_max_idle_per_host(2000)
        .tcp_keepalive(std::time::Duration::from_secs(60))
        .http2_keep_alive_interval(std::time::Duration::from_secs(30))
        .http2_keep_alive_timeout(std::time::Duration::from_secs(10))
        .http2_keep_alive_while_idle(true)
        .build()?;
    let mut rdr = Reader::from_path(input_file)?;
    let mut requests = Vec::new();
    
    for result in rdr.deserialize() {
        let request: ApiRequest = result?;
        requests.push(request);
    }

    // Calculate total requests
    let total_requests = requests.iter().map(|r| r.repetition).sum::<usize>();
    println!("Total requests to process: {}", total_requests);
    
    // Record start time for TPS calculation
    let start_time = Instant::now();
    
    // Create a stream of requests
    let mut stream = futures::stream::iter(requests)
        .flat_map(|request| {
            let client = client.clone();
            futures::stream::iter((0..request.repetition).map(move |_| {
                let endpoint = request.endpoint.clone();
                let method = request.method.clone();
                let body = request.body.clone();
                let client = client.clone();
                async move {
                    send_request(&client, &endpoint, &method, &body).await
                }
            }))
        })
        .buffer_unordered(2000);
    
    // Use a buffered writer for more efficient I/O
    let mut buffered_wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(std::io::BufWriter::new(std::fs::File::create(output_file)?));
    
    while let Some(task_result) = stream.next().await {
        match task_result {
            Ok(api_result) => buffered_wtr.serialize(api_result)?,
            Err(e) => eprintln!("Request failed: {}", e),
        }
    }
    buffered_wtr.flush()?;
    println!("Results written to {}", output_file);
    
    // Calculate and print TPS
    let total_time = start_time.elapsed();
    let tps = total_requests as f64 / total_time.as_secs_f64();
    println!("Total requests processed: {}", total_requests);
    println!("Total time: {:.2} seconds", total_time.as_secs_f64());
    println!("TPS: {:.2}", tps);
    
    Ok(())
}
