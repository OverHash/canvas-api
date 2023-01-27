use canvas_api::{CalendarExt, CanvasClient};

#[tokio::main]
async fn main() {
    let binding = std::fs::read_to_string(".env").expect("Failed to read .env");
    let canvas_token = binding
        .split_once('=')
        .and_then(|(_, value)| value.split('"').nth(1))
        .expect("Failed to parse env key")
        .to_string();

    let client = CanvasClient::builder(canvas_token)
        .set_api_url("https://[CANVAS_API]/api")
        .build()
        .expect("Failed to create canvas client");

    println!("{:?}", client.account_calendars(None).await);
}
