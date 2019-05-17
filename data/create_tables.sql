CREATE TABLE secret (
    digest TEXT NOT NULL,
    access_right INTEGER NOT NULL
);

CREATE TABLE music_score (
    title TEXT NOT NULL,
    file_name TEXT NOT NULL,
    other_files BLOB,
    lyrics TEXT,
    data_path_id INTEGER NOT NULL ,
    FOREIGN KEY(data_path_id) REFERENCES data_path(id)
);

create table data_path (
    id   INTEGER PRIMARY KEY,
    path TEXT NOT NULL
);

CREATE VIRTUAL TABLE music_index USING fts5(title, file_name, lyrics);