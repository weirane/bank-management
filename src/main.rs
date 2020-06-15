mod action;
mod error;
mod route;
mod types;

use actix_session::CookieSession;
use actix_web::middleware::{DefaultHeaders, Logger, NormalizePath};
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use rand::Rng as _;
use sqlx::mysql::MySqlPool;
use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    log::info!("Database URL: {}", db_url);
    let pool = MySqlPool::builder()
        .build(&db_url)
        .await
        .expect("Failed to create pool");
    let tera =
        tera::Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html")).unwrap();
    let priv_key: [u8; 32] = rand::thread_rng().gen();

    let bind = env::args().nth(1).unwrap_or_else(|| "0.0.0.0:8004".into());
    HttpServer::new(move || {
        use route::*;
        App::new()
            .wrap(NormalizePath)
            .wrap(DefaultHeaders::new().header("Content-Type", "text/html; charset=utf-8"))
            .wrap(
                CookieSession::signed(&priv_key)
                    .name("error")
                    .secure(false)
                    .same_site(actix_http::cookie::SameSite::Strict),
            )
            .wrap(Logger::default())
            .data(web::FormConfig::default().limit(4096))
            .data(pool.clone())
            .data(tera.clone())
            .service(index)
            .service(customer_add)
            .service(customer_del)
            .service(customer_change)
            .service(customer_query)
            .service(account_add)
            .service(account_del)
            .service(account_change)
            .service(account_query)
            .service(loan_add)
            .service(loan_del)
            .service(loan_issue)
            .service(loan_query)
            .service(stats)
            .service(add_customer)
            .service(del_customer)
            .service(query_customer)
            .service(add_account)
            .service(del_account)
            .service(change_account)
            .service(query_account)
            .service(add_loan)
            .service(del_loan)
            .service(issue_loan)
            .service(query_loan)
            .service(change_customer)
            .default_service(
                actix_files::Files::new("", "public").default_handler(
                    web::resource("")
                        .route(web::get().to(p404))
                        // all requests that are not GET
                        .route(
                            web::route()
                                .guard(guard::Not(guard::Get()))
                                .to(HttpResponse::MethodNotAllowed),
                        ),
                ),
            )
    })
    .bind(bind)?
    .run()
    .await
}
