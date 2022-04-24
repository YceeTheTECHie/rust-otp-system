#[tokio::main]
async fn main() {
    
    let result = reqwest::get("").await;
    println!("{:?}", result);
}

