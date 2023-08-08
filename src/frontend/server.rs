use actix_web::{get, App, HttpServer, HttpResponse };
use crate::frontend::Asset;

#[get("/css/{file:.*}")]
async fn serve_css(file: actix_web::web::Path<String>) -> HttpResponse {
    if let Some(data) = Asset::get(&format!("css/{}", file.into_inner())).map(|data| data.data.to_vec()) {
        HttpResponse::Ok().content_type("text/css").body(data)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/img/{file:.*}")]
async fn serve_img(file: actix_web::web::Path<String>) -> HttpResponse {
    if let Some(data) = Asset::get(&format!("img/{}", file.into_inner())).map(|data| data.data.to_vec()) {
        HttpResponse::Ok().content_type("image/png").body(data)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/")]
async fn home() -> HttpResponse {
    if let Some(data) = Asset::get("templates/home.html").map(|data| data.data.to_vec()) {
        HttpResponse::Ok().content_type("text/html").body(data)
    } else {
        HttpResponse::NotFound().finish()
    }
}

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(serve_css)
            .service(serve_img)
    })
    .bind("0.0.0.0:3030")?
    .run()
    .await
}