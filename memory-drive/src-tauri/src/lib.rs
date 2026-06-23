//! All the code in this file is human written
//! without any use of ai tools.
//! 
//! TODO remove unwraps

mod archive;

use std::{path::Path, time::Instant};

use crate::archive::{ArchiveMetadata, load_archive_metadata};

#[tauri::command]
fn load_archive(archive_path: &Path) -> ArchiveMetadata {
    let now = Instant::now();

    let archive_metadata = load_archive_metadata(archive_path);

    let elapsed = now.elapsed();
    println!("load_archive took: {:.2?}", elapsed);

    archive_metadata
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // let now = Instant::now();

    // {
    //     let ArchiveDirectories { month_directories, other_directories: _ } = analyse_archive(DEFAULT_ARCHIVE_PATH).unwrap();
    //     let mut all_imgs_metadata:Vec<ImgMetadata> = Vec::new();

    //     for m in &month_directories {
    //         let mut imgs = load_leaf_directory_file_metadatas(&m).unwrap();
    //         all_imgs_metadata.append(&mut imgs);
    //     }

    //     for m in &all_imgs_metadata {
    //         println!("{:?}", m);
    //     }

    //     for i in 0..10 {
    //         archive::create_thumbnail(&all_imgs_metadata[i]).unwrap();
    //     }

    //     println!("\r\n> Done");
    //     println!("Total vector length {:?}", all_imgs_metadata.len());
    // }

    // let elapsed = now.elapsed();
    // println!("Program execution took: {:.2?}", elapsed);
    // process::exit(0x0);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_archive])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
