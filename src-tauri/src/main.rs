#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use blake2::{Blake2b512, Blake2s256, Digest};
use serde::{ser::SerializeStruct, Serialize};
use std::{fs, os::windows::prelude::MetadataExt};
use tauri::{LogicalSize, Manager, Size};

struct HashResult {
    blake512: String,
    blake256: String,
}

struct FileStats {
    file_path: String,
    file_size: String,
    file_extenions: String,
    last_modified: String,
    file_name: String,
}

impl Serialize for HashResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("HashResult", 2)?;
        s.serialize_field("blake512", &self.blake512)?;
        s.serialize_field("blake256", &self.blake256)?;
        s.end()
    }
}

impl Serialize for FileStats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("FileStats", 5)?;
        s.serialize_field("file_path", &self.file_path)?;
        s.serialize_field("file_name", &self.file_name)?;
        s.serialize_field("file_size", &self.file_size)?;
        s.serialize_field("file_extension", &self.file_extenions)?;
        s.end()
    }
}

// Trys to read the file from the provide path
// returns blake2b512 hash as string
#[tauri::command]
fn check_file(file_path: String) -> HashResult {
    let file_bytes = fs::read(&file_path).unwrap();
    let hash_result = HashResult {
        blake256: get_blake2b_512(&file_bytes),
        blake512: get_blake2s_256(&file_bytes),
    };
    get_file_stats(&file_path);
    hash_result
}

/**
 * Generates Blake2b512 bit hash
 */
fn get_blake2b_512(input_bytes: &Vec<u8>) -> String {
    let mut hasher = Blake2b512::new();
    hasher.update(input_bytes);
    let res = hasher.finalize();
    dbg!("{?}", res);
    let result = format!("{:X}", res);
    result
}

/**
 * Generates Blake2s256 bit hash
 */
fn get_blake2s_256(input_bytes: &Vec<u8>) -> String {
    let mut hasher = Blake2s256::new();
    hasher.update(input_bytes);
    let res = hasher.finalize();
    dbg!("{?}", res);
    let result = format!("{:X}", res);
    result
}

fn get_file_stats(file_path: &str) {
    let metadata = fs::metadata(file_path).unwrap();
    let ex_name:(&str,&str) = get_file_name_and_ext(file_path);
    let file_stats = FileStats {
        file_path: String::from(file_path),
        file_size: metadata.len().to_string(),
        file_extenions: String::from(""),
        file_name : String::from("f"),
        last_modified: metadata.last_access_time().to_string()
    };
    println!("FileName {} {}",ex_name.0, ex_name.1)
}

fn get_file_name_and_ext(file_path: &str) -> (&str,&str) {
    let last_element = file_path.split('\\').last().unwrap();
    let split:Vec<&str> = last_element.split('.').collect();
    (split[0],split[1])
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            println!("Initializing");

            main_window
                .set_size(Size::Logical(LogicalSize {
                    width: 350.0,
                    height: 580.0,
                }))
                .unwrap();
            main_window.set_resizable(false).unwrap();
            println!("Done set size");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![check_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
