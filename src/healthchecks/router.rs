use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use tokio::time::{self, Duration, Interval};
use reqwest;



#[post("/health")]
pub async fn health(req_body: String) -> impl Responder {
    let url = if req_body.starts_with("http://") || req_body.starts_with("https://") {
        req_body.clone()
    } else {
        format!("http://{}", req_body)
    };
    tokio::spawn(async move {
        let mut interval: Interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            match reqwest::get(&url).await {
                Ok(response ) => {
                    println!("Response: {:?}", response);
                    if response.status().is_success() {
                        println!("Health check passed for {}", url);
                    } else {
                        println!("Health check failed for {}: {:?}", url, response.status());
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
    });

    HttpResponse::Ok().body(req_body)
}

