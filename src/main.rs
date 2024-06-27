
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()>{
    let res = reqwest::get("https://darknaija.com/").await?;
    println!("Response: {}", res.status());
    println!("Headers: {:#?}", res.headers());
    let body = res.text().await?;
    println!("Body: {}", body);
    Ok(())

}