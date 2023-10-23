// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod csv_extractor;

use csv_extractor::Data;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![extract_data_from_csv,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
    // let _ = extract_data_from_csv(
    // "/home/guillaume/Downloads/2023-10-11-981_segments_export_1697032083.csv",
    // );
    // let _ = extract_data_from_csv("/home/yehra/Downloads/convertcsv.csv");
}

#[tauri::command]
async fn extract_data_from_csv(file: &str) -> Result<Data, String> {
    let data = Data::new().extract_data_from_csv(file)?;
    Ok(data)
}
