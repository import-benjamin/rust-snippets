use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web_httpauth::extractors::{AuthenticationError, bearer};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web::dev::ServiceRequest;
use actix_web::error::Error;
use actix_web::get;

#[get("/")]
async fn home(auth: BearerAuth) -> String {
    format!("Hello, admin, {:?}", auth)
}

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // lookup value in 'Authorization' request header
    if credentials.token() == "some_token" { // fake Oauth token: Authorization="Bearer <token>"
        Ok(req) // Allow request
    } else {
        let config = req.app_data::<bearer::Config>()
            .cloned()
            .unwrap_or_default()
            .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");

        Err((AuthenticationError::from(config).into(), req))
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(std::time::SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    setup_logger().unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/admin").wrap(HttpAuthentication::bearer(validator)).service(home))
   })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}