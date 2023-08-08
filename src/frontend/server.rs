use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use actix_files as fs;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(include_str!("templates/home.html"))
}

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(fs::Files::new("/img", "src/frontend/img"))
            .service(fs::Files::new("/css", "src/frontend/css"))
    })
    // TODO : paramaterize this
    .bind("0.0.0.0:3030")?
    .run()
    .await
}
