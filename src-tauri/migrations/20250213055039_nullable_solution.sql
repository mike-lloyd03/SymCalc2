CREATE TABLE history_new (
    id integer primary key,
    equation text not null,
    solution real
);

INSERT INTO history_new SELECT * FROM history;

DROP TABLE history;

ALTER TABLE history_new RENAME TO history;
