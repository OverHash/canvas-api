# canvas-api

An implementation of the Instructure Canvas API.

### Usage

The canvas API is quite large. To mitigate API pollution, the core `CanvasClient` can have functions loaded by importing the relevant `Ext` trait.

For example, to use the `Account Calendar` APIs, import the `CalendarExt` trait:

```rs
use canvas_api::{CalendarExt, CanvasClient};

#[tokio::main]
async fn main() {
    let client = CanvasClient::builder("CANVAS_TOKEN")
        .build()
        .expect("Failed to create canvas client");

    println!("{:?}", client.account_calendars(None).await);
}
```
