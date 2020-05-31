use actix_session::Session;
use actix_web::{get, post, web, HttpResponse};
use futures::future::try_join_all;
use sqlx::mysql::MySqlPool;
use std::collections::HashMap;
use tera::Tera;

use crate::action::customer;
use crate::error::{Error, Result};
use crate::types::NewCustomer;

#[post("/customer/add")]
pub async fn add_customer(
    sess: Session,
    cus: web::Form<NewCustomer>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    if let Err(e) = cus.add(&pool).await {
        if let Error::Sqlx(sqlx::Error::Database(e)) = e {
            log::warn!("{}", e);
            let code = e
                .code()
                .map(|s| ": code ".to_owned() + s)
                .unwrap_or_default();
            sess.set("error_msg", format!("DB error{}", code)).ok();
        } else {
            return Err(e);
        }
    }
    Ok(HttpResponse::Found()
        .header("location", "/customer/add")
        .finish())
}

#[post("/customer/change")]
pub async fn change_customer(
    mut form: web::Form<HashMap<String, String>>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    let id = form.remove("id").ok_or(Error::BadRequest("no id"))?;
    let fs = form.iter().filter_map(|(k, v)| {
        let v = v.trim();
        if v.is_empty() {
            None
        } else {
            Some(customer::change(&id, k, v, &pool))
        }
    });
    try_join_all(fs).await?;
    Ok(HttpResponse::Found()
        .header("location", "/customer/change")
        .finish())
}

macro_rules! get_routes {
    ($($name:ident, $route:literal, $teml:literal;)*) => {
        $(
            #[get($route)]
            pub async fn $name(sess: Session, teml: web::Data<Tera>) -> Result<HttpResponse> {
                let mut ctx = tera::Context::new();
                if let Some(msg) = sess.get::<String>("error_msg")? {
                    sess.remove("error_msg");
                    ctx.insert("error_msg", &msg);
                }
                let s = teml.render($teml, &ctx)?;
                Ok(HttpResponse::Ok().body(s))
            }
        )*
    };
    ($($name:ident, $route:literal, $teml:literal);*) => {
        get_routes! { $($name, $route, $teml;)* }
    }
}

get_routes! {
    index, "/", "index.html";

    customer_add, "/customer/add", "customer/add.html";
    customer_del, "/customer/del", "customer/del.html";
    customer_change, "/customer/change", "customer/change.html";
    customer_query, "/customer", "customer/query.html";

    account_add, "/account/add", "account/add.html";
    account_del, "/account/del", "account/del.html";
    account_change, "/account/change", "account/change.html";
    account_query, "/account", "account/query.html";

    loan_add, "/loan/add", "loan/add.html";
    loan_del, "/loan/del", "loan/del.html";
    loan_issue, "/loan/issue", "loan/issue.html";
    loan_query, "/loan", "loan/query.html";

    stats, "/stats", "stats.html";
}

pub async fn p404(teml: web::Data<Tera>) -> HttpResponse {
    let s = teml
        .render("404.html", &tera::Context::new())
        .unwrap_or_else(|_| "404 Not Found".to_string());
    HttpResponse::NotFound().body(s)
}
