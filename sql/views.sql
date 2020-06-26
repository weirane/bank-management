drop view if exists checkaccounts;
drop view if exists saveaccounts;
drop view if exists checkstat;
drop view if exists savestat;
drop view if exists loanstat;
drop view if exists loan_with_paid;

create view checkaccounts
    (account_id, bank, type, balance, open_date, credit) as
select
    account.account_id, bank, type, balance, open_date, credit
from
    account left join checkacc using(account_id)
where type = 1;

create view saveaccounts
    (account_id, bank, type, balance, open_date, interest_rate, currency) as
select
    account.account_id, bank, type, balance, open_date, interest_rate, currency
from
    account left join saveacc using(account_id)
where type = 0;

create view checkstat(bank, total_balance, total_customer) as
select bank_name as bank, coalesce(total_balance, 0), coalesce(total_customer, 0)
from bank left join(
    select
        t1.bank, total_balance, total_customer
    from
        (
            select bank, sum(balance) as total_balance
            from checkaccounts
            group by bank
        ) t1 join
        (
            select bank, count(distinct customer_real_id) as total_customer
            from account left join has_account using(account_id)
            where type = 1
            group by bank
        ) t2 using(bank)
) t3 on bank.bank_name = t3.bank;

create view savestat(bank, total_balance, total_customer) as
select bank_name as bank, coalesce(total_balance, 0), coalesce(total_customer, 0)
from bank left join(
    select
        t1.bank, total_balance, total_customer
    from
        (
            select bank, sum(balance) as total_balance
            from saveaccounts
            group by bank
        ) t1 join
        (
            select bank, count(distinct customer_real_id) as total_customer
            from account left join has_account using(account_id)
            where type = 0
            group by bank
        ) t2 using(bank)
) t3 on bank.bank_name = t3.bank;

create view loanstat(bank, total_amount, total_customer) as
select bank_name as bank, coalesce(total_amount, 0), coalesce(total_customer, 0)
from bank left join(
    select
        t1.bank, total_amount, total_customer
    from
        (
            select bank, sum(amount) as total_amount
            from loan
            group by bank
        ) t1 join
        (
            select bank, count(distinct customer_real_id) as total_customer
            from loan, make_loan
            where loan.loan_id = make_loan.loan_id
            group by bank
        ) t2 using(bank)
) t3 on bank.bank_name = t3.bank;

create view loan_with_paid(loan_id, amount, bank, state, paid) as
select loan_id, amount, bank, state, paid
from loan left join(
    select loan_id, coalesce(sum(loan_pay.amount),0) as paid
    from loan left join loan_pay
    using(loan_id)
    group by loan_id
) t using(loan_id);
