extern crate core;

mod handlers;
mod utils;

use std::env;

use actix_web::{middleware, web, App, HttpServer};

use actix_web_middleware_keycloak_auth::{DecodingKey, KeycloakAuth};

use futures::future;

const INTERNAL_PORT: &str = "8080";
const PUBLIC_PORT: &str = "8081";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load all env vars
    // uncomment to run locally and comment before creating the docker image.
    //     dotenv::dotenv().unwrap();

    env_logger::init();

    println!("Internal sever listening on port: {}", INTERNAL_PORT);
    println!("Public server listening on port: {}", PUBLIC_PORT);

    let internal_server = HttpServer::new(move || {
        let keycloak_auth = KeycloakAuth::default_with_pk(
            DecodingKey::from_rsa_pem(env::var("KEYCLOAK_PUBLIC_KEY").as_ref().unwrap().as_bytes())
                .unwrap(),
        );

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(keycloak_auth)
            .service(handlers::upload)
            .service(handlers::list)
            .service(handlers::delete)
            .service(utils::compress)
            .service(utils::decompress)
            .service(handlers::index_protected)
    })
    .bind(format!("0.0.0.0:{}", INTERNAL_PORT))?
    .run();

    let public_server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::exclude(
                middleware::Logger::default(),
                "/health/health_check",
            ))
            .service(
                web::scope("/health").route("/health_check", web::get().to(handlers::health_check)),
            )
            .service(handlers::index)
    })
    .bind(format!("0.0.0.0:{}", PUBLIC_PORT))?
    .run();

    future::try_join(internal_server, public_server).await?;
    Ok(())
}
