// Test it with the following commands:
// curl -X DELETE http://localhost:8080/datafile.txt
// curl -X GET http://localhost:8080/datafile.txt
// curl -X PUT http://localhost:8080/datafile.txt -d "File contents."
// curl -X POST http://localhost:8080/data -d "File contents."
// curl -X GET http://localhost:8080/a/b
//
// after running the second command, the client should have printed:
// Contents of the file.
//
// After running all five commands, the server should have printed:
// Listening at address 127.0.0.1:8080 ...
// Deleting file "datafile.txt" ... Deleted file "datafile.txt"
// Downloading file "datafile.txt" ... Downloaded file "datafile.txt"
// Uploading file "datafile.txt" ... Uploaded file "datafile.txt"
// Uploading file "data_*.txt" ... Uploaded file "data_17.txt"
// Invalid URI: "/a/b"

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
