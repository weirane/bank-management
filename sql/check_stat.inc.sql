select bank_name as bank,
       coalesce(total_loanpay, 0) as total_loanpay,
       coalesce(total_customer, 0) as total_customer
from bank left join(
    select
        t1.bank, total_loanpay, total_customer
    from
        (
            select bank, loan_id, sum(loan_pay.amount) as total_loanpay
            from loan right join loan_pay using(loan_id)
            where paytime < ?
            group by loan_id
        ) t1 join
        (
            select bank, count(distinct customer_id) as total_customer
            from (loan right join loan_pay using(loan_id))
                 left join make_loan using(loan_id)
            where paytime < ?
            group by(bank)
        ) t2 using(bank)
) t3 on bank.bank_name = t3.bank;
