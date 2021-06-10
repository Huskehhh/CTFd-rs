create table scoreboard (
    entry_id int auto_increment,
    ctf_id int not null,
    points int not null,
    position text not null,
    entry_time datetime default CURRENT_TIMESTAMP not null,
    constraint scoreboard_entry_id_uindex unique (entry_id)
);
alter table scoreboard
add primary key (entry_id);