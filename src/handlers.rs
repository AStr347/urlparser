use actix_web::http::header::ContentType;
use actix_web::web::Data;
use actix_web::{route, HttpResponse, HttpRequest};
use playwright::api::BrowserContext;
use std::time::{SystemTime, UNIX_EPOCH};

fn bad_res(body: String) -> HttpResponse {
    let response = HttpResponse::InternalServerError()
        .content_type(ContentType::plaintext())
        .body(body);
    return response;
}



#[route("/", method = "GET", method = "HEAD", method = "POST")]
///
/// GET and HEAD handler
/// always return HttpResponse with "check_health" text
///
pub async fn home() -> HttpResponse {
    let response = HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("SB2 url's parser home");
    return response;
}

#[route("/fetch", method = "POST")]
///
/// GET and HEAD handler
/// always return HttpResponse with "check_health" text
///
pub async fn fetch(req: HttpRequest, context: Data<BrowserContext>) -> HttpResponse {
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let url_res = req.headers().get("url");
    if url_res.is_none(){
        let err_line = line!();
        return bad_res(format!("/fetch:{err_line} POST error: headers have not 'url'"));
    }
    let url = url_res.unwrap().to_str().unwrap();

    let page_res = context.new_page().await;
    if let Err(e) = page_res{
        let err_line = line!();
        return bad_res(format!("/fetch:{err_line} POST error: {e}"));
    }
    let page = page_res.unwrap();
    let goto_res = page.goto_builder(url).goto().await;
    if let Err(e) = goto_res{
        let err_line = line!();
        return bad_res(format!("/fetch:{err_line} POST error: {e}"));
    }


    let content_res = page.inner_text("body", None).await;
    if let Err(e) = content_res{
        let err_line = line!();
        return bad_res(format!("/fetch:{err_line} POST error: {e}"));
    }
    let content = content_res.unwrap();

    let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let d = end_time - start_time;
    println!("fetch time diff: {:?}us", d);


    let response = HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(content);

    let _ = page.close(None).await;

    return response;
}
