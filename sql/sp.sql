drop procedure if exists add_save_account;
drop procedure if exists add_check_account;

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
