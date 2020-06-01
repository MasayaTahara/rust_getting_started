use reqwest::StatusCode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://httpbin.org/ip";
    println!("call {}", url);
    if let Ok(resp) = reqwest::get(url).await {
        match resp.status() {
            StatusCode::OK => {
                let body = resp.text().await?;
                println!("response is \n{}", body);        
            },
            StatusCode::NOT_FOUND => {
                println!("404 not found");
            },
            _ => {
                println!("unknown error");
            },
        }
    } else {
        println!("server not found");
    }
    Ok(())
}
