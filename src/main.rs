use actix_files::NamedFile;
use actix_web::{get, App, HttpResponse, HttpServer, Result, Responder};
use actix_web::http::header::{ContentDisposition, DispositionType};
use log::info;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/file")]
async fn file() -> Result<NamedFile> {
    info!("{}", "about to serve the file");
    let file = NamedFile::open("docs/magna-carta.pdf")?;
    Ok(file
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
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
