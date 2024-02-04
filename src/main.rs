// use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::path::Path;
use walkdir::WalkDir;

mod libparse;

// #[get("/")]
// async fn hello() -> impl Responder {
// 	HttpResponse::Ok().body("hello")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
// 	HttpServer::new(|| {
// 		App::new()
// 			.service(hello)
// 	})
// 	.bind(("127.0.0.1", 8080))?
// 	.run()
// 	.await
// }

fn main() {
	// WalkDir Rust
	// -----------------
	// real    0m5.989s
	// user    0m0.622s
	// sys     0m2.746s

	// find /home/nathan
	// -----------------
	// real    0m5.608s
	// user    0m0.525s
	// sys     0m3.020s
	let path = Path::new("/home/nathan");
	let walker = WalkDir::new(path);

	for res in walker {
		let _entry = match res {
			Ok(entry) => {
				println!("{}", entry.path().display());
			},
			Err(err) => {
				println!("ERROR: {}", err);
				continue;
			}
		};
	}
}