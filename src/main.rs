use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address).expect("Failed to start in port 80000");
    run(listener)?.await
}
