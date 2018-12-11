use jsonwebtoken as jwt;

mod controllers;
mod handlers;
mod middlewares;
mod routes;

fn main() {
    // log setting
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let sys = actix_web::actix::System::new("oxiblog");
    let port = "0.0.0.0:3000";
    actix_web::server::new(|| routes::app())
        .bind(port)
        .unwrap()
        .start();
    println!("start server at: {}", port);
    let _ = sys.run();
}
