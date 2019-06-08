use std::ffi::{OsStr, OsString};
use std::os::raw::c_uint;
use std::path::{Path, PathBuf};
use std::process::Command;

use walkdir::{DirEntry, WalkDir};
use std::fs::create_dir_all;

enum MusicStyle {
    Agnus = 1,
    Alleluia = 2,
    Acclamations = 3,
    Common = 4,
    Gloria = 5,
    Kyrie = 6,
    Psaumes = 7,
    Sanctus = 8,
    Anamnese = 9,
    Taize = 10,
    Messes = 11,
}

struct DbEntry {
    title: String,
    file_name: String,
    style: Vec<i8>,
    other_files: Option<Vec<String>>,
    lyrics: String,
    data_path_id: u8,
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

    let pdftotext = Command::new("pdftotext")
        .arg("-enc")
        .arg("UTF-8")
        .arg(file_entry.path())
        .arg("-")
        .output()
        .expect("failed to execute pdftotext process.");

    let lyrics = String::from_utf8(pdftotext.stdout).expect("");

    trace!("pdftotext command result {:?}.", lyrics);

    let title_raw = file_entry.path()
        .file_name()
        .expect("Cannot convert file name")
        .to_os_string();

    let title = String::from(title_raw.to_str().expect("Cannot convert to &str"));
    let style: Vec<i8> = find_music_style(&lyrics, &title, &title).into_iter().map(|music_style: MusicStyle| music_style as i8).collect();
    let file_name = title.clone();

    if !lyrics.trim().is_empty() {
        return Some(
            DbEntry {
                title,
                file_name,
                style,
                other_files: None,
                lyrics,
                data_path_id: 1,
            })
    }
    let tmp_directory = String::from("/tmp/mtsundays/");
    create_dir_all(&tmp_directory)
        .expect("Creation of working tmp dir fails.");

    trace!("Extracting image from {:?}", file_entry.path());
    let file_name_str = title_raw.to_str().expect("Cannot convert path to string.");

    let pdfimages = Command::new("pdfimages")
        .arg("-j")
        .arg(file_entry.path())
        .arg(format!("{}{}", &tmp_directory, file_name_str))
        .output()
        .expect("Failed to execute pdfimages process.");

    for entry in WalkDir::new(tmp_directory) {
        let file_entry: DirEntry = entry.expect("Cannot get result entry.");
        let image_extracted_name = file_entry.file_name().to_str().expect("Cannot convert file_name to str.");

        if !image_extracted_name.contains(file_name_str) {
            trace!("Image {} does not need to be converted.", image_extracted_name);
            continue;
        };

        trace!("Found image {:?} to convert.", file_entry.path());
        let tesseract = Command::new("tesseract")
            .arg(file_entry.path())
            .arg("stdout")
            .output()
            .expect("Failed to execute tesseract process.");

        trace!("Tesseract command result {:?}.", lyrics);
        let lyrics = String::from_utf8(tesseract.stdout);
    };

    None
}

fn create_from_picture(file_entry: DirEntry) -> Option<DbEntry> {
    debug!("Found PICTURE entry {:?}.", file_entry);

    let tesseract = Command::new("tesseract")
        .arg(file_entry.path())
        .arg("stdout")
        .output()
        .expect("Failed to execute tesseract process.");

    let lyrics = String::from_utf8(tesseract.stdout);

    trace!("Tesseract command result {:?}.", lyrics);
    let title_raw = file_entry.path()
        .file_name()
        .expect("Cannot convert file name")
        .to_os_string();

    let title = String::from(title_raw.to_str().expect("Cannot convert to &str"));
    let lyrics = lyrics.expect("Cannot find lyrics.");
    let style: Vec<i8> = find_music_style(&lyrics, &title, &title).into_iter().map(|music_style: MusicStyle| music_style as i8).collect();
    let file_name = title.clone();

    Some(
        DbEntry {
            title,
            file_name,
            style,
            other_files: None,
            lyrics,
            data_path_id: 1,
        })
}

fn find_music_style(lyrics: &str, file_name: &str, title: &str) -> Vec<MusicStyle> {
    vec![MusicStyle::Acclamations]
}