use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.service(hello)
			.service(echo)
			.route("/hey", web::get().to(manual_hello))
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
