drop table if exists items;

create table items (
    done          boolean default 0,
    name          text not null,
    priority      tinyint not null,
    description   text,
    deadline      text,
    id            integer primary key autoincrement
);

insert into items
(name, priority, description, deadline)
 values (
    "get groceries",
    50,
    null,
    "2022-06-30"
);

insert into items
(name, priority, description, deadline)
 values (
    "some generic thing",
    150,
    null,
    "2022-06-28"
);

insert into items
(name, priority, description, deadline)
 values (
    "some other generic thing",
    175,
    null,
    "2022-06-28"
);
