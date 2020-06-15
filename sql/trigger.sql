drop trigger if exists account_type;

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
