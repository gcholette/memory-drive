//! All the code in this file is human written
//! without any use of ai tools.

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
            let mut imgs = load_leaf_directory_file_metadatas(&m, DEFAULT_ARCHIVE_PATH).unwrap();
            all_imgs_metadata.append(&mut imgs);
        }

        for m in &all_imgs_metadata {
            println!("{:?}", m);
        }

        println!("\r\n> Done");
        println!("Total vector length {:?}", all_imgs_metadata.len());
    }

    let elapsed = now.elapsed();
    println!("Loading metadata took: {:.2?}", elapsed);
    process::exit(0x0);

    // let sleep_time = time::Duration::from_millis(100000);
    // thread::sleep(sleep_time);

    // let _ = create_thumbnail("");

    //tauri::Builder::default()
        //.plugin(tauri_plugin_opener::init())
        // .invoke_handler(tauri::generate_handler![analyse_archive])
        //.run(tauri::generate_context!())
        //.expect("error while running tauri application");
}
