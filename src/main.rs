use actix_web::{web::Data, App, HttpServer};
use playwright::Playwright;

mod handlers;
use crate::handlers::*;

// #[tokio::main]
// async fn main() -> Result<(), playwright::Error> {
//     let playwright = Playwright::initialize().await?;
//     playwright.prepare()?; // Install browsers
//     let chromium = playwright.chromium();
//     let browser = chromium.launcher().headless(true).launch().await?;
//     let context = browser.context_builder().build().await?;
//     let page = context.new_page().await?;
//     page.goto_builder("https://example.com/").goto().await?;

//     let content = page.content().await?;
//     println!("{content}");

//     Ok(())
// }


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
