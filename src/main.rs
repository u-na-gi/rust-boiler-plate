
use actix_web::{ post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;

use runacy_note_backend::routes::*;


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Loggerの初期化
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let server = HttpServer::new(|| {
        App::new()
            // ミドルウェアとしてロガーを登録
            .wrap(actix_web::middleware::Logger::default())
            .service(health_check)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    });

    let address = "127.0.0.1:8080";
    println!("Server running at http://{}", address);

    server
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
