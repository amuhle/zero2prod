use zero2prod::configuration;
use zero2prod::startup::Application;
use zero2prod::telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Use info-level or above if RUST_LOG env var has not been set
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into());
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
