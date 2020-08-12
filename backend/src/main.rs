#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

mod cli_args;
mod comment;
mod database;
mod errors;
mod feedback;
mod graphql;
mod jwt;
mod participant;
mod schema;
mod user;

use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let opt = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };
    let schema = std::sync::Arc::new(crate::graphql::model::create_schema());
    let domain = opt.domain.clone();
    let cookie_secret_key = opt.auth_secret_key.clone();
    let secure_cookie = opt.secure_cookie;
    let auth_duration = chrono::Duration::hours(i64::from(opt.auth_duration_in_hour));
    let port = opt.port;
    let pool = web::Data::new(database::pool::establish_connection(opt.clone()));
    match database::init_db::init_data(pool.clone()) {
        _ => {}
    }

    let server = HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .data(schema.clone())
            .data(opt.clone())
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(cookie_secret_key.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(&domain)
                    .max_age_time(auth_duration)
                    .secure(secure_cookie),
            ))
            .service(web::scope("/api").configure(user::route))
            .configure(graphql::route)
    })
    .bind(("0.0.0.0", port))
    .unwrap()
    .run();

    eprintln!("Listening on 0.0.0.0:{}", port);

    server.await
}
