use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    //"https://github.com/Pirate-Weather/pirateweather"
    let response = reqwest::get("https://api.pirateweather.net/forecast/5CNI3wn3PL2L6wIia7aX9RqMdyIA4ARJ/52.520008,13.404954").await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("Response: {}", body);
    } else {
        println!("Failed to get a successful response: {}", response.status());
    }

    Ok(())
}
