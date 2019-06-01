use core::borrow::Borrow;
use std::path::PathBuf;

use walkdir::{DirEntry, WalkDir};

enum MusicStyle {
    Agnus,
    Alleluia,
    Acclamations,
    Common,
    Gloria,
    Kyrie,
    Psaumes,
    Sanctus,
    Anamnese,
    Taize,
    Messes,
}

struct DbEntry {
    title: String,
    file_name: String,
    style: Vec<u8>,
    other_files: Vec<String>,
    lyrics: String,
    data_path_id: i8,
}

pub fn create_database(input: PathBuf, output_db_name: PathBuf, data_path: PathBuf) {
    debug!("Starting creation of database using input directory {:?}, data files path {:?} and output db name {:?}.", input, data_path, output_db_name);

    for entry in WalkDir::new(input) {
        trace!("Found entry {:?}.", entry);
        let db_entry = create_db_entry(entry);
    }
}

fn create_db_entry(entry: Result<DirEntry, walkdir::Error>) -> Option<DbEntry> {
    let file_entry: DirEntry = entry.expect("Cannot get result entry.");

    if file_entry.file_type().is_dir() {
        return None;
    }
    trace!("Found file {:?}.", file_entry);

    let extension_entry = file_entry.path().extension();
    if extension_entry.is_none() {
        return None;
    }

    let extension_lowercase = extension_entry.expect("Cannot found extension.")
        .to_str()
        .expect("Cannot convert OsString extension into str.")
        .to_lowercase();

    trace!("Extension name is {}.", extension_lowercase);

    match extension_lowercase.as_str() {
        "pdf" => create_from_pdf(file_entry),
        "jpg" | "tiff" | "webp" | "png" | "jpeg" => create_from_picture(file_entry),
        _ => None
    }
}

fn create_from_pdf(file_entry: DirEntry) -> Option<DbEntry> {
    debug!("Found PDF entry {:?}.", file_entry);

    None
}

fn create_from_picture(file_entry: DirEntry) -> Option<DbEntry> {
    debug!("Found PICTURE entry {:?}.", file_entry);

    None
}