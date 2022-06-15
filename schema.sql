create table items (
    done          boolean default 0,
    name          text not null,
    priority      tinyint not null,
    description   text,
    deadline      text,
    id            integer primary key autoincrement
);
