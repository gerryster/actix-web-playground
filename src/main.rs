use actix_files::NamedFile;
use actix_web::{get, App, Error, HttpResponse, HttpRequest, HttpServer, Result, Responder};
use actix_web::http::header::{self, ContentDisposition, DispositionType, HeaderValue};
use log::info;

#[get("/")]
async fn root(req: HttpRequest) -> impl Responder {
    info!("request uri: {}", req.uri());
    HttpResponse::Ok().body("Hello world!")
}

#[get("/file-no-store")]
async fn file(req: HttpRequest) -> Result<HttpResponse, Error> {
    info!("{}", "file-no-cache hit");
    let file = NamedFile::open("docs/magna-carta.pdf")?;
    let mut response = file
        .use_etag(false)
        .use_last_modified(false)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        })
        .into_response(&req);

    let headers = response.headers_mut();
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));

    Ok(response)
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
