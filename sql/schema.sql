create table bank (
    bank_name varchar(32) not null,
    city varchar(32) not null,
    asset decimal(14, 2) not null default 0,
    constraint PK_BANK primary key (bank_name)
);

create table department (
    depart_id int not null,
    name varchar(32) not null,
    type varchar(32) not null,
    bank varchar(20) not null,
    constraint PK_DEPARTMENT primary key (depart_id),
    constraint FK_BANK_DEPART foreign key (bank) references bank(bank_name)
);

create table employee (
    emp_id char(18) not null,
    name varchar(32) not null,
    tel char(20) not null,
    address varchar(128) not null,
    start_date date not null,
    depart int not null,
    manager char(18),  -- null: this employee is the manager
    constraint PK_EMPLOYEE primary key (emp_id),
    constraint FK_DEPART_EMP foreign key (depart) references department(depart_id),
    constraint FK_MANAGER_EMP foreign key (manager) references employee(emp_id)
);

create table contacter (
    contacter_id int not null,
    name varchar(32) not null,
    tel char(20) not null,
    email varchar(32) not null,
    constraint PK_CONTACTER primary key (contacter_id)
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
    constraint PK_CUSTOMER primary key (customer_id),
    constraint FK_CUS_CONTACT foreign key (contacter_id) references contacter(contacter_id),
    constraint FK_CUS_LOANRES foreign key (loan_emp) references employee(emp_id),
    constraint FK_CUS_ACCRES foreign key (deposit_emp) references employee(emp_id)
);

create table account (
    account_id char(6) not null,
    balance decimal(12, 2) not null default 0,
    open_date date not null,
    type int not null,
    bank varchar(20) not null,
    constraint PK_ACC primary key (account_id),
    constraint CK_ACC_TYPE check(type in (0, 1))  -- 0: save, 1: check
);

create table saveacc (
    account_id char(6) not null,
    interest_rate decimal(5, 2) not null,
    currency char(3) not null,
    constraint PK_SAVEACC primary key (account_id),
    constraint FK_SAVE_ACC foreign key (account_id) references account(account_id) on delete cascade
);

create table checkacc (
    account_id char(6) not null,
    credit decimal(12, 2) not null default 0,
    constraint PK_CHECKACC primary key (account_id),
    constraint FK_CHECK_ACC foreign key (account_id) references account(account_id) on delete cascade
);

create table has_account (
    account_id char(6) not null,
    customer_id char(18) not null,
    last_visit datetime not null default current_timestamp on update current_timestamp,
    constraint PK_HAS_ACCOUNT primary key (account_id, customer_id),
    constraint FK_CUS_HAS foreign key (customer_id) references customer(customer_id),
    constraint FK_ACC_HAS foreign key (account_id) references account(account_id) on delete cascade
);

create table loan (
    loan_id char(11) not null,
    amount decimal(12, 2) not null,
    bank varchar(20) not null,
    state int not null default 0,
    constraint PK_LOAN primary key (loan_id),
    constraint FK_LOAN_BANK foreign key (bank) references bank(bank_name)
);

create table make_loan (
    loan_id char(11) not null,
    customer_id char(18) not null,
    constraint PK_MAKE_LOAN primary key (loan_id, customer_id),
    constraint PK_MKLOAN_LOAN foreign key (loan_id) references loan(loan_id) on delete cascade,
    constraint PK_MKLOAN_CUS foreign key (customer_id) references customer(customer_id)
);

create table loan_pay (
    loan_id char(11) not null,
    amount decimal(12, 2) not null,
    paytime datetime not null default current_timestamp,
    constraint PK_LOAN_PAY primary key (loan_id, amount, paytime),
    constraint FK_LOANPAY_LOAN foreign key (loan_id) references loan(loan_id) on delete cascade
);
