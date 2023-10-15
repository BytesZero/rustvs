use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

#[get("/fibonacci/{n}")]
async fn calculate_fibonacci(path: web::Path<u32>) -> impl Responder {
    let n = path.into_inner();
    let result = fibonacci(n);
    HttpResponse::Ok().body(format!("Fibonacci({}) = {}", n, result))
}

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(calculate_fibonacci)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
