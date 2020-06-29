# bank-management

Lab 3 of Database Systems in Spring 2020. A web application built with
[actix-web] and [sqlx].

[actix-web]: https://github.com/actix/actix-web
[sqlx]: https://github.com/launchbadge/sqlx

## Setup

Setup database, run in [mycli]:

```sql
create database bank
use bank
source ./sql/all.sql
```

Compile and run

```sh
export DATABASE_URL=mysql://user:pass@127.0.0.1/bank
cargo run --release
```

[mycli]: https://github.com/dbcli/mycli
