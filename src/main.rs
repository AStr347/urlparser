use actix_web::{web::Data, App, HttpServer};
use playwright::Playwright;

mod handlers;
use crate::handlers::*;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let pw = Playwright::initialize().await.unwrap();
    let chromium = pw.chromium();
    let browser = chromium.launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();

    let actix_data = Data::new(context);
    
    HttpServer::new(move || {
        App::new()
            //.app_data(app_state.clone())
            .service(home)
            .service(fetch)
            .app_data(actix_data.clone())
    })
    .bind(("0.0.0.0", 65535))?
    .run()
    .await
}
