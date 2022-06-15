create table session (id blob not null primary key, user_id integer not null, foreign key (user_id) references strapi_administrator (id));

create table collections (id integer primary key, name string not null, slug string not null, table_name string not null);