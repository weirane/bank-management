drop procedure if exists add_save_account;
drop procedure if exists add_check_account;
drop procedure if exists add_has_account;
drop procedure if exists add_make_loan;

create procedure add_save_account(
    in account_id char(6), in bank varchar(32), in balance decimal(12, 2),
    in currency char(3), in interest_rate decimal(5, 2)
) begin
    insert into account(account_id, balance, open_date, type, bank)
    values (account_id, balance, curdate(), 0, bank);

    insert into saveacc(account_id, interest_rate, currency)
    values (account_id, interest_rate, currency);
end;

create procedure add_check_account(
    in account_id char(6), in bank varchar(32), in balance decimal(12, 2),
    in credit decimal(12, 2)
) begin
    insert into account(account_id, balance, open_date, type, bank)
    values (account_id, balance, curdate(), 1, bank);

    insert into checkacc(account_id, credit)
    values (account_id, credit);
end;

create procedure add_has_account(in account_id char(6), in customer_id char(18)) begin
    declare real_id int;
    select customer_real_id into real_id from customer
    where customer.customer_id = customer_id
    limit 1;
    insert into has_account(account_id, customer_real_id)
    values (account_id, real_id);
end;

create procedure add_make_loan(in loan_id char(11), in customer_id char(18)) begin
    declare real_id int;
    select customer_real_id into real_id from customer
    where customer.customer_id = customer_id
    limit 1;
    insert into make_loan(loan_id, customer_real_id)
    values (loan_id, real_id);
end;
