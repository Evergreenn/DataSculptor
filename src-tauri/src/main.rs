// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // tauri::Builder::default()
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
    process_file("/home/guillaume/Downloads/2023-10-11-981_segments_export_1697032083.csv");
}

fn process_file(file: &str) -> Result<(), std::io::Error> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open(file).expect("file not found");
    let mut reader = BufReader::new(file).lines();
    let mut string_buf: Vec<String> = Vec::new();

    //TODO: in each line, split by comma, and create a hashmap with the position in list as key and the value as value
    //Iterator::position() to get the index of the value in the header

    while let Some(line) = reader.next() {
        let line = line.unwrap();
        string_buf.push(line);
    }
    let header_string = string_buf[0].clone();
    let doc = string_buf[1..].to_vec();

    println!("header: {}", header_string);
    println!("doc: {:#?}", doc);
    Ok(())
}
