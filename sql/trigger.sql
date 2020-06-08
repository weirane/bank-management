drop trigger if exists account_type;

create trigger account_type
before insert on has_account
for each row begin
    declare n int;
    select count(account_id) into n from saveaccounts left join has_account using(account_id)
    where customer_id = new.customer_id and type = 0;
    if n != 0 then
        signal sqlstate '45000' set message_text = '储蓄账户已存在';
    end if;
    select count(account_id) into n from checkaccounts left join has_account using(account_id)
    where customer_id = new.customer_id and type = 1;
    if n != 0 then
        signal sqlstate '45000' set message_text = '支票账户已存在';
    end if;
end;
