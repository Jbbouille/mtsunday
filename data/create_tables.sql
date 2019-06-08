CREATE TABLE secret
(
    digest       TEXT    NOT NULL,
    access_right INTEGER NOT NULL
);

CREATE TABLE music_score
(
    title        TEXT    NOT NULL,
    file_name    TEXT    NOT NULL,
    style        TEXT,
    other_files  BLOB,
    lyrics_id    INTEGER NOT NULL,
    data_path_id INTEGER NOT NULL,
    FOREIGN KEY (lyrics_id) REFERENCES music_style (id),
    FOREIGN KEY (data_path_id) REFERENCES data_path (id)
);

create table music_style
(
    id    INTEGER PRIMARY KEY,
    style TEXT NOT NULL
);

create table data_path
(
    id   INTEGER PRIMARY KEY,
    path TEXT NOT NULL
);

CREATE VIRTUAL TABLE music_index USING fts5
(
    title,
    file_name,
    lyrics,
    style
);