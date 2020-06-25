use actix_session::Session;
use actix_web::{get, post, web, HttpResponse};
use chrono::{Datelike, NaiveDate};
use futures::future::try_join_all;
use serde_json::json;
use sqlx::mysql::{MySqlPool, MySqlQueryAs, MySqlRow};
use sqlx::Row;
use std::collections::HashMap;
use std::iter;
use tera::Tera;

use crate::action::{account, customer, loan};
use crate::error::{Error, Result};
use crate::types::{Customer, Loan};
use crate::types::{LoanStat, SaveStat};
use crate::types::{NewAccount, NewCustomer, NewLoan, NewLoanPay};

pub type SMap = HashMap<String, String>;

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
    match customer::del(&id, &pool).await {
        Ok(0) => {
            sess.set("error_msg", "客户不存在".to_owned()).ok();
        }
        Err(Error::Sqlx(sqlx::Error::Database(e))) => {
            log::warn!("{}", e);
            let msg = db_error_msg!(e,
                "23000" =>
                    contains "FK_CUS_HAS" : |_| "此客户有关联账户"
                    contains "FK_MKLOAN_CUS" : |_| "此客户有贷款记录"
            );
            sess.set("error_msg", msg).ok();
        }
        Err(e) => return Err(e),
        _ => (),
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
    form: web::Form<SMap>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    let empty = String::new();
    let ret: Vec<Customer> = sqlx::query_as!(
        Customer,
        "select customer_id as id, name, tel,
            address, cast(contacter_id as char(10)) as contacter_id, relation
        from customer where
        customer_id like concat('%', ?, '%')
        and name like concat('%', ?, '%')
        and tel like concat('%', ?, '%')
        and address like concat('%', ?, '%')
        and contacter_id like concat('%', ?, '%')
        and relation like concat('%', ?, '%')
        ",
        form.get("id").unwrap_or(&empty),
        form.get("name").unwrap_or(&empty),
        form.get("tel").unwrap_or(&empty),
        form.get("address").unwrap_or(&empty),
        form.get("contacter_id").unwrap_or(&empty),
        form.get("relation").unwrap_or(&empty),
    )
    .fetch_all(&**pool)
    .await?;
    Ok(HttpResponse::Ok().json(ret))
}

#[post("/account/add")]
pub async fn add_account(
    sess: Session,
    form: web::Json<NewAccount>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    if let Err(e) = form.add(&pool).await {
        if let Error::Sqlx(sqlx::Error::Database(e)) = e {
            log::warn!("{}", e);
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
    Ok(HttpResponse::Ok().finish())
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
    form: web::Form<SMap>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    let atype = form.get("type").ok_or(Error::BadRequest("type"))?;
    let ret = match atype.as_str() {
        "" => {
            // any
            let save = account::query_save(&form, &pool).await?;
            let check = account::query_check(&form, &pool).await?;
            let values: Vec<serde_json::Value> = save
                .iter()
                .map(serde_json::to_value)
                .chain(check.iter().map(serde_json::to_value))
                .collect::<std::result::Result<_, serde_json::Error>>()?;
            serde_json::to_value(values)?
        }
        "0" => {
            // save
            account::query_save(&form, &pool)
                .await
                .and_then(|v| serde_json::to_value(v).map_err(Into::into))?
        }
        "1" => {
            // check
            account::query_check(&form, &pool)
                .await
                .and_then(|v| serde_json::to_value(v).map_err(Into::into))?
        }
        _ => return Err(Error::BadRequest("type")),
    };
    Ok(HttpResponse::Ok().json(ret))
}

#[post("/loan/add")]
pub async fn add_loan(
    sess: Session,
    form: web::Json<NewLoan>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    if let Err(e) = form.add(&pool).await {
        if let Error::Sqlx(sqlx::Error::Database(e)) = e {
            log::warn!("{}", e);
            let msg = db_error_msg!(e,
                "23000" =>
                    starts_with "Duplicate entry" : |_| "贷款号已存在"
            );
            sess.set("error_msg", msg).ok();
        } else {
            return Err(e);
        }
    }
    Ok(HttpResponse::Ok().finish())
}

#[post("/loan/del")]
pub async fn del_loan(
    sess: Session,
    form: web::Form<SMap>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    let id = form.get("id").ok_or(Error::BadRequest("no id"))?;
    match loan::del(id, &pool).await {
        Ok(0) => {
            sess.set("error_msg", "贷款号不存在".to_owned()).ok();
        }
        Err(Error::Sqlx(sqlx::Error::Database(e))) => {
            log::warn!("{}", e);
            let msg = db_error_msg!(e,
                "45003" =>
                    eq "贷款发放中" : |m| m
            );
            sess.set("error_msg", msg).ok();
        }
        Err(e) => return Err(e),
        _ => (),
    }
    Ok(HttpResponse::Found()
        .header("location", "/loan/del")
        .finish())
}

#[post("/loan/issue")]
pub async fn issue_loan(
    sess: Session,
    form: web::Form<NewLoanPay>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    if let Err(e) = form.add(&pool).await {
        if let Error::Sqlx(sqlx::Error::Database(e)) = e {
            log::warn!("{}", e);
            let msg = db_error_msg!(e,
                "23000" =>
                    contains "FK_LOANPAY_LOAN" : |_| "ID 不存在"
                "45002" =>
                    eq "超出贷款金额" : |m| m
            );
            sess.set("error_msg", msg).ok();
        } else {
            return Err(e);
        }
    }
    Ok(HttpResponse::Found()
        .header("location", "/loan/issue")
        .finish())
}

#[post("/loan/query")]
pub async fn query_loan(form: web::Form<SMap>, pool: web::Data<MySqlPool>) -> Result<HttpResponse> {
    let empty = String::new();
    let ret: Vec<Loan> = sqlx::query_as!(
        Loan,
        "select loan_id as id, bank, amount, paid,
            substr('未开始发放正在发放 已全部发放', state*5+1, 5) as state
        from loan_with_paid where
        loan_id like concat('%', ?, '%')
        and bank like concat('%', ?, '%')
        and amount like concat('%', ?, '%')
        and state like concat('%', ?, '%')
        ",
        form.get("id").unwrap_or(&empty),
        form.get("bank").unwrap_or(&empty),
        form.get("amount").unwrap_or(&empty),
        form.get("state").unwrap_or(&empty),
    )
    .fetch_all(&**pool)
    .await?;
    Ok(HttpResponse::Ok().json(ret))
}

fn gen_dates(years: i32) -> impl Iterator<Item = NaiveDate> {
    let today = chrono::Utc::today().naive_utc();
    let (mut y, mut m) = (today.year(), today.month());
    let start = NaiveDate::from_ymd(y, m, 1);
    let ys = y;
    iter::once(start).chain(iter::from_fn(move || {
        if m == 1 {
            y -= 1;
            m = 12;
        } else {
            m -= 1;
        }
        if y <= ys - years {
            None
        } else {
            Some(NaiveDate::from_ymd(y, m, 1))
        }
    }))
}

#[post("/stats/save")]
pub async fn stats_save_data(pool: web::Data<MySqlPool>) -> Result<HttpResponse> {
    let mut datas = Vec::new();
    for date in gen_dates(3) {
        // sqlx::query_as!(Foo, include_str!("...")) is not available
        // launchbadge/sqlx#388
        let s: Vec<SaveStat> = sqlx::query_as(include_str!("../sql/save_stat.inc.sql"))
            .bind(date)
            .bind(date)
            .fetch_all(&**pool)
            .await?;
        datas.push(json!({ date.to_string(): s }));
        if s.iter().all(|st| st.no_business()) {
            break;
        }
    }
    let banks = sqlx::query("select bank_name from bank")
        .map(|x: MySqlRow| -> String { x.get("bank_name") })
        .fetch_all(&**pool)
        .await?;
    let ret = json!({
        "banks": banks,
        "datas": datas
    });
    Ok(HttpResponse::Ok().json(ret))
}

#[post("/stats/check")]
pub async fn stats_check_data(pool: web::Data<MySqlPool>) -> Result<HttpResponse> {
    let mut datas = Vec::new();
    for date in gen_dates(3) {
        let s: Vec<LoanStat> = sqlx::query_as(include_str!("../sql/check_stat.inc.sql"))
            .bind(date)
            .bind(date)
            .fetch_all(&**pool)
            .await?;
        datas.push(json!({ date.to_string(): s }));
        if s.iter().all(|st| st.no_business()) {
            break;
        }
    }
    let banks = sqlx::query("select bank_name from bank")
        .map(|x: MySqlRow| -> String { x.get("bank_name") })
        .fetch_all(&**pool)
        .await?;
    let ret = json!({
        "banks": banks,
        "datas": datas
    });
    Ok(HttpResponse::Ok().json(ret))
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

get_routes!(customer_add, "/customer/add", "customer/add.html", {
    let contacters = sqlx::query("select cast(contacter_id as char(10)) as id from contacter")
        .map(|x: MySqlRow| -> String { x.get("id") })
        .fetch_all(&**pool)
        .await?;
    ctx.insert("contacters", &contacters);
});
get_routes!(customer_del, "/customer/del", "customer/del.html", {
    let customers = sqlx::query("select customer_id from customer")
        .map(|x: MySqlRow| -> String { x.get("customer_id") })
        .fetch_all(&**pool)
        .await?;
    ctx.insert("customers", &customers);
});
get_routes!(customer_change, "/customer/change", "customer/change.html");
get_routes!(customer_query, "/customer", "customer/query.html");

get_routes!(account_add, "/account/add", "account/add.html", {
    let banks = sqlx::query("select bank_name from bank")
        .map(|x: MySqlRow| -> String { x.get("bank_name") })
        .fetch_all(&**pool)
        .await?;
    let customers = sqlx::query("select customer_id from customer")
        .map(|x: MySqlRow| -> String { x.get("customer_id") })
        .fetch_all(&**pool)
        .await?;
    ctx.insert("banks", &banks);
    ctx.insert("customers", &customers);
});
get_routes!(account_del, "/account/del", "account/del.html", {
    let accounts = sqlx::query("select account_id from account")
        .map(|x: MySqlRow| -> String { x.get("account_id") })
        .fetch_all(&**pool)
        .await?;
    ctx.insert("accounts", &accounts);
});
get_routes!(account_change, "/account/change", "account/change.html", {
    let banks = sqlx::query("select bank_name from bank")
        .map(|x: MySqlRow| -> String { x.get("bank_name") })
        .fetch_all(&**pool)
        .await?;
    ctx.insert("banks", &banks);
});
get_routes!(account_query, "/account", "account/query.html");

get_routes!(loan_add, "/loan/add", "loan/add.html", {
    let banks = sqlx::query("select bank_name from bank")
        .map(|x: MySqlRow| -> String { x.get("bank_name") })
        .fetch_all(&**pool)
        .await?;
    let customers = sqlx::query("select customer_id from customer")
        .map(|x: MySqlRow| -> String { x.get("customer_id") })
        .fetch_all(&**pool)
        .await?;
    ctx.insert("banks", &banks);
    ctx.insert("customers", &customers);
});
get_routes!(loan_del, "/loan/del", "loan/del.html", {
    let loans = sqlx::query("select loan_id from loan")
        .map(|x: MySqlRow| -> String { x.get("loan_id") })
        .fetch_all(&**pool)
        .await?;
    ctx.insert("loans", &loans);
});
get_routes!(loan_issue, "/loan/issue", "loan/issue.html");
get_routes!(loan_query, "/loan", "loan/query.html");

get_routes!(stats_save, "/stats/save", "stats/save.html");
get_routes!(stats_check, "/stats/check", "stats/check.html");

pub async fn p404(teml: web::Data<Tera>) -> HttpResponse {
    let s = teml
        .render("404.html", &tera::Context::new())
        .unwrap_or_else(|_| "404 Not Found".to_string());
    HttpResponse::NotFound().body(s)
}
