use actix_session::Session;
use actix_web::{get, post, web, HttpResponse};
use futures::future::try_join_all;
use sqlx::mysql::{MySqlPool, MySqlRow};
use sqlx::Row;
use std::collections::HashMap;
use tera::Tera;

use crate::action::{account, customer};
use crate::error::{Error, Result};
use crate::types::{NewAccount, NewCustomer, NewLoan, TypedNewAccount};

type SMap = HashMap<String, String>;

#[post("/customer/add")]
pub async fn add_customer(
    sess: Session,
    cus: web::Form<NewCustomer>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    if let Err(e) = cus.add(&pool).await {
        if let Error::Sqlx(sqlx::Error::Database(e)) = e {
            log::warn!("{}", e);
            let msg = if let Some(c) = e.code() {
                if c == "23000" {
                    if e.message().contains("FK_CUS_CONTACT") {
                        "联系人不存在".to_owned()
                    } else if e.message().starts_with("Duplicate entry") {
                        "客户已存在".to_owned()
                    } else {
                        "DB error: code 23000".to_owned()
                    }
                } else {
                    format!("DB error: code {}", c)
                }
            } else {
                "DB error".to_owned()
            };
            sess.set("error_msg", msg).ok();
        } else {
            return Err(e);
        }
    }
    Ok(HttpResponse::Found()
        .header("location", "/customer/add")
        .finish())
}

#[post("/customer/del")]
pub async fn del_customer(
    sess: Session,
    form: web::Form<SMap>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    let id = form.get("id").ok_or(Error::BadRequest("no id"))?;
    let n = customer::del(&id, &pool).await?;
    if n == 0 {
        sess.set("error_msg", "客户不存在".to_owned()).ok();
    }
    Ok(HttpResponse::Found()
        .header("location", "/customer/del")
        .finish())
}

#[post("/customer/change")]
pub async fn change_customer(
    mut form: web::Form<SMap>,
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

#[post("/customer/query")]
pub async fn query_customer(
    sess: Session,
    form: web::Form<SMap>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    eprintln!("{:#?}", form);
    // TODO
    Ok(HttpResponse::Found()
        .header("location", "/customer")
        .finish())
}

#[post("/account/add")]
pub async fn add_account(
    sess: Session,
    form: web::Form<NewAccount>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    use std::convert::TryInto;
    let acc: TypedNewAccount = form.into_inner().try_into()?;
    if let Err(e) = acc.add(&pool).await {
        if let Error::Sqlx(sqlx::Error::Database(e)) = e {
            log::warn!("{}", e);
            eprintln!("{:#?}", e);
            let msg = if let Some(c) = e.code() {
                match c {
                    "23000" => {
                        if e.message().starts_with("Duplicate entry") {
                            "账户号已存在".to_owned()
                        } else {
                            "DB error: code 23000".to_owned()
                        }
                    }
                    // TODO: 45000 account existed
                    _ => format!("DB error: code {}", c),
                }
            } else {
                "DB error".to_owned()
            };
            sess.set("error_msg", msg).ok();
        } else {
            return Err(e);
        }
    }
    Ok(HttpResponse::Found()
        .header("location", "/account/add")
        .finish())
}

#[post("/account/del")]
pub async fn del_account(
    sess: Session,
    form: web::Form<SMap>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    let id = form.get("id").ok_or(Error::BadRequest("no id"))?;
    let n = account::del(id, &pool).await?;
    if n == 0 {
        sess.set("error_msg", "账户不存在".to_owned()).ok();
    }
    Ok(HttpResponse::Found()
        .header("location", "/account/del")
        .finish())
}

#[post("/account/change")]
pub async fn change_account(
    sess: Session,
    form: web::Form<SMap>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    eprintln!("{:#?}", form);
    // TODO
    Ok(HttpResponse::Found()
        .header("location", "/account/change")
        .finish())
}

#[post("/account/query")]
pub async fn query_account(
    sess: Session,
    form: web::Form<SMap>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    eprintln!("{:#?}", form);
    // TODO
    Ok(HttpResponse::Found()
        .header("location", "/account")
        .finish())
}

#[post("/loan/add")]
pub async fn add_loan(
    sess: Session,
    form: web::Form<NewLoan>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    eprintln!("{:#?}", form);
    // TODO
    Ok(HttpResponse::Found()
        .header("location", "/loan/add")
        .finish())
}

#[post("/loan/del")]
pub async fn del_loan(
    sess: Session,
    form: web::Form<SMap>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    eprintln!("{:#?}", form);
    // TODO
    Ok(HttpResponse::Found()
        .header("location", "/loan/del")
        .finish())
}

#[post("/loan/issue")]
pub async fn issue_loan(
    sess: Session,
    form: web::Form<SMap>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    eprintln!("{:#?}", form);
    // TODO
    Ok(HttpResponse::Found()
        .header("location", "/loan/issue")
        .finish())
}

#[post("/loan/query")]
pub async fn query_loan(
    sess: Session,
    form: web::Form<SMap>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    eprintln!("{:#?}", form);
    // TODO
    Ok(HttpResponse::Found().header("location", "/loan").finish())
}

macro_rules! get_routes {
    ($name:ident, $route:literal, $teml:literal, $b:block) => {
        #[get($route)]
        pub async fn $name(
            sess: Session,
            teml: web::Data<Tera>,
            pool: web::Data<MySqlPool>
        ) -> Result<HttpResponse> {
            let mut ctx = tera::Context::new();
            if let Some(msg) = sess.get::<String>("error_msg")? {
                sess.remove("error_msg");
                ctx.insert("error_msg", &msg);
            }
            $b
            let s = teml.render($teml, &ctx)?;
            Ok(HttpResponse::Ok().body(s))
        }
    };
    ($name:ident, $route:literal, $teml:literal) => {
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
    };
}

get_routes!(index, "/", "index.html");

get_routes!(customer_add, "/customer/add", "customer/add.html");
get_routes!(customer_del, "/customer/del", "customer/del.html");
get_routes!(customer_change, "/customer/change", "customer/change.html");
get_routes!(customer_query, "/customer", "customer/query.html");

get_routes!(account_add, "/account/add", "account/add.html", {
    let banks = sqlx::query("select bank_name from bank")
        .map(|x: MySqlRow| -> String { x.get("bank_name") })
        .fetch_all(&**pool)
        .await?;
    ctx.insert("banks", &banks);
});
get_routes!(account_del, "/account/del", "account/del.html");
get_routes!(account_change, "/account/change", "account/change.html");
get_routes!(account_query, "/account", "account/query.html");

get_routes!(loan_add, "/loan/add", "loan/add.html");
get_routes!(loan_del, "/loan/del", "loan/del.html");
get_routes!(loan_issue, "/loan/issue", "loan/issue.html");
get_routes!(loan_query, "/loan", "loan/query.html");

get_routes!(stats, "/stats", "stats.html");

pub async fn p404(teml: web::Data<Tera>) -> HttpResponse {
    let s = teml
        .render("404.html", &tera::Context::new())
        .unwrap_or_else(|_| "404 Not Found".to_string());
    HttpResponse::NotFound().body(s)
}
