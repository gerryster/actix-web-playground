use actix_files::NamedFile;
use actix_web::{get, App, HttpResponse, HttpServer, Result, Responder};
use log::info;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/file")]
async fn file() -> Result<NamedFile> {
    info!("{}", "about to serve the file");
    Ok(NamedFile::open("docs/magna-carta.pdf")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(file)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
