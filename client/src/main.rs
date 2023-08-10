use reqwest::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    // let response = client.get("http://localhost:3000/questions")
    //     .send()
    //     .await?;

    // let body = response.text().await?;
    // println!("{}", body);

    // let response = client.get("http://localhost:3000/questions")
    //     .send()
    //     .await?;

    // let body = response.text().await?;
    // println!("{}", body);

    // let response = client.post("http://localhost:3000/question")
    //     .send()
    //     .await?;

    // let body = response.text().await?;
    // println!("{}", body);

    Ok(())
}
