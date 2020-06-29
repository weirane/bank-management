delete from department;
delete from employee;
delete from has_account;
delete from saveacc;
delete from checkacc;
delete from account;
delete from make_loan;
delete from loan_pay;
update loan set state = 0;
delete from loan;
delete from customer;
delete from contacter;
delete from bank;

insert into contacter values
    (1, 'c1', 'ct1', 'ce1'),
    (2, 'c2', 'ct2', 'ce2'),
    (3, 'c3', 'ct3', 'ce3')
    ;

alter table customer auto_increment = 1;

insert into customer (customer_id, contacter_id, name, tel, address, relation) values
    ('429001', 1, 'cname',     '123456789', 'caddr1', 'cr1'),
    ('429002', 1, 'cname2',    '109284712', 'caddr2', 'cr2'),
    ('429003', 2, 'ahdj',      '123820581', 'caddr3', 'cr3'),
    ('429004', 1, 'alfio',     '101148291', 'caddr4', 'cr4'),
    ('429005', 3, 'afsfj s',   '109921948', 'caddr5', 'cr5'),
    ('429006', 1, 'f20j alfj', '109427138', 'caddr6', 'cr6'),
    ('429007', 2, 'abcdalwi',  '123750928', 'caddr7', 'cr7')
    ;

insert into bank values
    ('Bank1', 'city', 12.34),
    ('Bank2', 'city2', 34.56),
    ('Bank3', 'city3', 56.78)
    ;

insert into account values
    ('s0001', 123, '2018-10-02', 0, 'Bank1'),
    ('s0002', 456, '2019-02-01', 0, 'Bank3'),
    ('s0003', 567, '2020-03-01', 0, 'Bank2'),
    ('s0004', 678, '2020-06-01', 0, 'Bank1'),
    ('s0005', 789, '2020-03-01', 0, 'Bank2'),

    ('c0001', 789, '2019-01-02', 1, 'Bank1'),
    ('c0002', 678, '2019-06-02', 1, 'Bank1'),
    ('c0003', 567, '2020-05-02', 1, 'Bank1'),
    ('c0004', 456, '2020-01-02', 1, 'Bank2'),
    ('c0005', 456, '2020-06-02', 1, 'Bank2')
    ;

insert into saveacc values
    ('s0001', 0.12, 'CNY'),
    ('s0002', 0.45, 'USD'),
    ('s0003', 0.55, 'USD'),
    ('s0004', 0.65, 'USD'),
    ('s0005', 0.75, 'USD')
    ;

insert into checkacc values
    ('c0001', 1000),
    ('c0002', 10000),
    ('c0003', 100),
    ('c0004', 4000),
    ('c0005', 1000)
    ;

insert into has_account (account_id, customer_real_id) values
    ('s0001', 1),
    ('s0002', 1),
    ('s0002', 2),
    ('s0004', 3),
    ('s0004', 4),
    ('s0005', 5),
    ('c0001', 1),
    ('c0002', 2),
    ('c0003', 3),
    ('c0004', 4)
    ;

insert into loan (loan_id, amount, bank) values
    ('L0001', 12345, 'Bank1'),
    ('L0002', 987, 'Bank2'),
    ('L0003', 54321, 'Bank1'),
    ('L0004', 10, 'Bank3'),
    ('L0005', 192.12, 'Bank3')
    ;

insert into make_loan (loan_id, customer_real_id) values
    ('L0001', 1),
    ('L0002', 1),
    ('L0003', 1),
    ('L0001', 2),
    ('L0004', 2),
    ('L0002', 3)
    ;

insert into loan_pay (loan_id, amount, paytime) values
    ('L0001', 123, '2018-04-02'),
    ('L0001', 123, '2019-02-02'),
    ('L0001', 223, '2019-07-11'),
    ('L0002', 90, '2020-01-04'),
    ('L0002', 297, '2020-04-05'),
    ('L0003', 97, '2020-01-04'),
    ('L0005', 90, '2020-05-06')
    ;
