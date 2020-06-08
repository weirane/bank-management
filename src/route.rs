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

macro_rules! db_error_msg {
    ($err:expr, $($code:literal => $($test:ident $str:literal : $msg:expr)+)+) => {{
        let e = $err;
        if let Some(c) = e.code() {
            match c {
                $($code => match e.message() {
                    $(m if m.$test($str) => ($msg)(m).to_string(),)+
                    _ => concat!("DB error: code ", $code).to_owned(),
                })+,
                _ => format!("DB error: code {}", c),
            }
        } else {
            "DB error".to_owned()
        }
    }};
}

#[post("/customer/add")]
pub async fn add_customer(
    sess: Session,
    cus: web::Form<NewCustomer>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    if let Err(e) = cus.add(&pool).await {
        if let Error::Sqlx(sqlx::Error::Database(e)) = e {
            log::warn!("{}", e);
            let msg = db_error_msg!(e,
                "23000" =>
                    contains "FK_CUS_CONTACT" : |_| "联系人不存在"
                    starts_with "Duplicate entry" : |_| "客户已存在"
            );
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
            let msg = db_error_msg!(e,
                "23000" =>
                    starts_with "Duplicate entry" : |_| "账户号已存在"
                "45000" =>
                    ends_with "已存在" : |m| m
            );
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
    mut form: web::Form<SMap>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    let id = form.remove("id").ok_or(Error::BadRequest("no id"))?;
    let atype = form.remove("type").ok_or(Error::BadRequest("no type"))?;
    if let Some(v) = form.remove("bank") {
        if !v.is_empty() {
            sqlx::query!("update account set bank = ? where account_id = ?", v, id)
                .execute(&**pool)
                .await?;
        }
    }
    if let Some(v) = form.remove("balance") {
        if !v.is_empty() {
            sqlx::query!("update account set balance = ? where account_id = ?", v, id)
                .execute(&**pool)
                .await?;
        }
    }
    match atype.as_str() {
        "Save" => {
            let fs = form.iter().filter_map(|(k, v)| {
                let v = v.trim();
                if v.is_empty() {
                    None
                } else {
                    Some(account::change_save(&id, k, v, &pool))
                }
            });
            try_join_all(fs).await?;
        }
        "Check" => {
            let fs = form.iter().filter_map(|(k, v)| {
                let v = v.trim();
                if v.is_empty() {
                    None
                } else {
                    Some(account::change_check(&id, k, v, &pool))
                }
            });
            try_join_all(fs).await?;
        }
        _ => return Err(Error::BadRequest("wrong type")),
    };
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
get_routes!(account_change, "/account/change", "account/change.html", {
    let banks = sqlx::query("select bank_name from bank")
        .map(|x: MySqlRow| -> String { x.get("bank_name") })
        .fetch_all(&**pool)
        .await?;
    ctx.insert("banks", &banks);
});
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
