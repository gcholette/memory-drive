//! All the code in this file is human written
//! without any use of ai tools.
//! 
//! TODO remove unwraps

mod archive;

use std::{process, time::Instant};
use crate::archive::{ArchiveAnalysis, ImgMetadata, analyse_archive, load_leaf_directory_file_metadatas};

const DEFAULT_ARCHIVE_PATH: &str = "/home/gcholette/Pictures/mdrive_test/";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let now = Instant::now();

    {
        let ArchiveAnalysis { month_directories, other_directories: _ } = analyse_archive(DEFAULT_ARCHIVE_PATH).unwrap();
        let mut all_imgs_metadata:Vec<ImgMetadata> = Vec::new();

        for m in &month_directories {
            let mut imgs = load_leaf_directory_file_metadatas(&m).unwrap();
            all_imgs_metadata.append(&mut imgs);
        }

        for m in &all_imgs_metadata {
            println!("{:?}", m);
        }

        for i in 0..10 {
            archive::create_thumbnail(&all_imgs_metadata[i]).unwrap();
        }

        println!("\r\n> Done");
        println!("Total vector length {:?}", all_imgs_metadata.len());
    }

    let elapsed = now.elapsed();
    println!("Program execution took: {:.2?}", elapsed);
    process::exit(0x0);

    //tauri::Builder::default()
        //.plugin(tauri_plugin_opener::init())
        // .invoke_handler(tauri::generate_handler![analyse_archive])
        //.run(tauri::generate_context!())
        //.expect("error while running tauri application");
}
