#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use blake2::{Blake2b512, Blake2s256, Digest};
use serde::{ser::SerializeStruct, Serialize};
use std::fs;
use tauri::{LogicalSize, Manager, Size};

struct HashResult {
    blake512: String,
    blake256: String,
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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
// Trys to read the file from the provide path
// returns blake2b512 hash as string

#[tauri::command]
fn check_file(file_path: String) -> HashResult {
    let file_bytes = fs::read(file_path).unwrap();
    let hash_result = HashResult {
        blake256: get_blake2b_512(&file_bytes),
        blake512: get_blake2s_256(&file_bytes),
    };
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
        .invoke_handler(tauri::generate_handler![greet, check_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
