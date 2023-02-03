#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs};

use blake2::{Blake2b512, Blake2s256, Digest};
use tauri::{LogicalSize, Manager, Size};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
 // Trys to read the file from the provide path
 // returns blake2b512 hash as string 

#[tauri::command]
fn check_file(file_path: String) -> String {
    let file_bytes = fs::read(file_path);
    let mut hasher = Blake2b512::new();
    hasher.update(file_bytes.unwrap());
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
