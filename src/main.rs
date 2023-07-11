use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[actix_web::main] // or #[tokio::main]
async fn main() {
    let _ = HttpServer::new(|| App::new().service(get))
        .bind(("0.0.0.0", 8080))
        .expect("failed to start Server")
        .run()
        .await;
}

#[get("/")]
async fn get(req: HttpRequest) -> impl Responder {
    let peer_addr = if let Some(peer_addr) = req.peer_addr() {
        peer_addr.ip().to_string()
    } else {
        return HttpResponse::BadRequest().body("failed");
    };

    return HttpResponse::Ok().body(peer_addr);
}
