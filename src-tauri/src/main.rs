// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

fn main() {
    // tauri::Builder::default()
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
    let _ = process_file("/home/yehra/Downloads/convertcsv.csv");
}

fn process_file(file: &str) -> Result<(HashMap<&str, Vec<HashMap<usize, &str>>>), std::io::Error> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open(file).expect("file not found");
    let mut reader = BufReader::new(file).lines();
    let mut string_buf: Vec<String> = Vec::new();

    while let Some(line) = reader.next() {
        let line = line.unwrap();
        string_buf.push(line);
    }
    let header = string_buf[0].clone();
    let mut doc = string_buf[1..].to_vec();
    let mut doc_indexed: Vec<HashMap<usize, &str>> = vec![];

    for full_line in doc.iter_mut() {
        let e = full_line
            .split(",")
            .enumerate()
            .collect::<HashMap<usize, &str>>();

        doc_indexed.push(e.to_owned());
    }

    let header = header
        .split(",")
        .enumerate()
        .collect::<HashMap<usize, &str>>();

    // println!("header: {:?}", header);
    // println!("{:#?}", doc_indexed);

    let mut result = HashMap::new();
    result.insert("header", vec![header]);
    result.insert("data", doc_indexed.clone());

    Ok(result)
}
