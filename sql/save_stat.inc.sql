select bank_name as bank,
       coalesce(total_balance, 0) as total_balance,
       coalesce(total_customer, 0) as total_customer
from bank left join(
    select
        t1.bank, total_balance, total_customer
    from
        (
            select bank, sum(balance) as total_balance
            from account
            where open_date < ?
            group by bank
        ) t1 join
        (
            select bank, count(distinct customer_id) as total_customer
            from account left join has_account using(account_id)
            where open_date < ?
            group by bank
        ) t2 using(bank)
) t3 on bank.bank_name = t3.bank;
