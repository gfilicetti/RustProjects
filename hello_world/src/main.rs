use reqwest::Url;

#[tokio::main]
async fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("Expect a command line argument but didn't get one");
    let url = Url::parse(&arg).expect("You didn't pass me a valid url");
    let response = reqwest::get(url).await.expect("Couldn't reach the internet");
    assert!(response.status() == 200, "Did not get a 200 OK");
    println!("{}", response.text().await.expect("Could not read the response body"));
}