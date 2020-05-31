create table bank (
    bank_name varchar(32) not null,
    city varchar(32) not null,
    asset decimal(14, 2) not null default 0,
    primary key (bank_name)
);

create table department (
    depart_id int not null,
    name varchar(32) not null,
    type varchar(32) not null,
    bank varchar(20) not null,
    primary key (depart_id),
    foreign key (bank) references bank(bank_name)
);

create table employee (
    emp_id char(18) not null,
    name varchar(32) not null,
    tel char(20) not null,
    address varchar(128) not null,
    start_date date not null,
    depart int not null,
    manager char(18),  -- null: this employee is the manager
    primary key (emp_id),
    foreign key (depart) references department(depart_id),
    foreign key (manager) references employee(emp_id)
);

create table contacter (
    contacter_id int not null,
    name varchar(32) not null,
    tel char(20) not null,
    email varchar(32) not null,
    primary key (contacter_id)
);

create table customer (
    customer_id char(18) not null,
    name varchar(32) not null,
    tel char(20) not null,
    address varchar(128) not null,
    contacter_id int not null,
    relation varchar(10) not null,
    loan_emp char(18),
    deposit_emp char(18),
    primary key (customer_id),
    foreign key (contacter_id) references contacter(contacter_id),
    foreign key (loan_emp) references employee(emp_id),
    foreign key (deposit_emp) references employee(emp_id)
);

create table account (
    account_id char(6) not null,
    balance decimal(12, 2) not null default 0,
    open_date date not null,
    type int not null,
    primary key (account_id),
    constraint check(type in (0, 1))  -- 0: save, 1: check
);

create table saveacc (
    account_id char(6) not null,
    interest_rate decimal(5, 2) not null,
    currency char(3) not null,
    primary key (account_id),
    foreign key (account_id) references account(account_id) on delete cascade
);

create table checkacc (
    account_id char(6) not null,
    credit decimal(12, 2) not null default 0,
    primary key (account_id),
    foreign key (account_id) references account(account_id) on delete cascade
);

create table has_account (
    account_id char(6) not null,
    customer_id char(18) not null,
    last_visit datetime not null default current_timestamp on update current_timestamp,
    bank varchar(20) not null,
    type int not null,
    primary key (account_id, customer_id),
    foreign key (bank) references bank(bank_name),
    foreign key (customer_id) references customer(customer_id),
    foreign key (account_id) references account(account_id) on delete cascade,
    constraint check(type in (0, 1)),  -- 0: save, 1: check
    constraint unique key(bank, customer_id, type)
);

create table loan (
    loan_id char(11) not null,
    amount decimal(12, 2) not null,
    bank varchar(20) not null,
    state int not null default 0,
    primary key (loan_id),
    foreign key (bank) references bank(bank_name)
);

create table make_loan (
    loan_id char(11) not null,
    customer_id char(18) not null,
    primary key (loan_id, customer_id),
    foreign key (loan_id) references loan(loan_id) on delete cascade,
    foreign key (customer_id) references customer(customer_id)
);

create table loan_pay (
    loan_id char(11) not null,
    amount decimal(12, 2) not null,
    paytime datetime not null default current_timestamp,
    primary key (loan_id, amount, paytime),
    foreign key (loan_id) references loan(loan_id) on delete cascade
);
