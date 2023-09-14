use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello World!")
// }

#[post("/echo")]
async fn echo(request_body: String) -> impl Responder {
    HttpResponse::Ok().body(request_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web Test!")
            }))
            .service(index)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}