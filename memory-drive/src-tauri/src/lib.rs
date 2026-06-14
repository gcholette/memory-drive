use std::fs;

#[tauri::command]
fn load_img(file_path: &str) -> Vec<u8> {
    let image_data:Vec<u8> = fs::read(file_path)
    .expect("Should have been able to read the file");

    image_data
}

// fn load_imgs_in_path(folder_path: &str) -> Vec<Vec<u8>> {

// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_img])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
