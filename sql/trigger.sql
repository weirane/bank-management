drop trigger if exists check_overloan;
drop trigger if exists loan_state;
drop trigger if exists loan_delete;
drop trigger if exists account_type;

create trigger check_overloan
before insert on loan_pay
for each row begin
    declare pay int;
    declare total int;
    select sum(amount) into pay from loan_pay where loan_pay.loan_id=new.loan_id;
    select amount into total from loan where loan.loan_id=new.loan_id;
    if pay + new.amount > total then
        signal sqlstate '45002' set message_text = '超出贷款金额';
    end if;
end;

create trigger loan_state
after insert on loan_pay
for each row begin
    declare pay int;
    declare total int;
    select sum(amount) into pay from loan_pay where loan_pay.loan_id=new.loan_id;
    select amount into total from loan where loan.loan_id=new.loan_id;
    if pay > 0 and pay < total then
        update loan set state='1' where loan.loan_id=new.loan_id;
    elseif pay = total then
        update loan set state='2' where loan.loan_id=new.loan_id;
    elseif pay > total then
        signal sqlstate '45002' set message_text = '超出贷款金额';
    end if;
end;

create trigger loan_delete
before delete on loan
for each row begin
    declare a int;
    select state into a from loan where old.loan_id=loan.loan_id;
    if a = 1 then
        signal sqlstate '45003' set message_text = '贷款发放中';
    end if;
end;

create trigger account_type
before insert on has_account
for each row begin
    declare n int;
    declare t int;
    declare b varchar(32);
    select type, bank into t, b from account where account_id = new.account_id;
    if t is null then
        signal sqlstate '45001' set message_text = '账户不存在';
    end if;
    if t = 0 then
        select count(account_id) into n from account right join has_account using(account_id)
        where customer_id = new.customer_id and account.bank = b and type = 0;
        if n != 0 then
            signal sqlstate '45000' set message_text = '储蓄账户已存在';
        end if;
    elseif t = 1 then
        select count(account_id) into n from account right join has_account using(account_id)
        where customer_id = new.customer_id and account.bank = b and type = 1;
        if n != 0 then
            signal sqlstate '45000' set message_text = '支票账户已存在';
        end if;
    end if;
end;
