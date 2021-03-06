use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const ADDR: &str = "127.0.0.1:8080";
    println!("{}", ADDR);
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(
                // prefixes all resources and routes attached to it...
                web::scope("/app")
                    // ...so this handles requests for `GET /app/index.html`
                    .route("/index.html", web::get().to(index)),
            )
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(ADDR)?
        .run()
        .await
    //workers(4)
}
