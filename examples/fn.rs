use feignhttp_rs::{get, post};

#[get("https://api.github.com")]
async fn github() -> feignhttp_rs::Result<String> {}

#[post("https://httpbin.org/post")]
async fn post() -> feignhttp_rs::Result<String> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = github().await?;
    println!("github result: {}", r);

    let r = post().await?;
    println!("post result: {}", r);

    Ok(())
}
