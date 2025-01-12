use actix_web::{get, App, web, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

mod types;
use types::{Dimension, Hint};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .service(generate_nonogram)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

#[get("/generate_nonogram")]
async fn generate_nonogram(query: web::Query<Dimension>) -> impl Responder {
    let hints: Hint;
    if query.x == 5 && query.y == 5 {
        hints = Hint {x_hints: vec!(vec!(1, 3), vec!(3), vec!(5), vec!(3), vec!(1, 1)), y_hints: vec!(vec!(1, 3), vec!(3), vec!(5), vec!(3), vec!(1, 1))};
    } else if query.x == 10 && query.y == 10 {
        hints = Hint {x_hints: vec!(vec!(2), vec!(4), vec!(6), vec!(8), vec!(10), vec!(10), vec!(8), vec!(6), vec!(4), vec!(2)), y_hints: vec!(vec!(2), vec!(4), vec!(6), vec!(8), vec!(10), vec!(10), vec!(8), vec!(6), vec!(4), vec!(2))};
    } else {
        hints = Hint {x_hints: vec!(vec!(1), vec!(3), vec!(5), vec!(7), vec!(9), vec!(11), vec!(13), vec!(15), vec!(13), vec!(11), vec!(9), vec!(7), vec!(5), vec!(3), vec!(1)), y_hints: vec!(vec!(1), vec!(3), vec!(5), vec!(7), vec!(9), vec!(11), vec!(13), vec!(15), vec!(13), vec!(11), vec!(9), vec!(7), vec!(5), vec!(3), vec!(1))};
    }
    HttpResponse::Ok().json(hints)
}
