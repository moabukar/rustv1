use actix_files::NamedFile;
use actix_web::{web, App, HttpServer};

async fn index() -> Result<NamedFile, E> {
    Ok::<NamedFile, E>::open("static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
