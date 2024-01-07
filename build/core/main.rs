use client::App;

#[tokio::main]
async fn main() {
    server::serve_app(App).await;
}
