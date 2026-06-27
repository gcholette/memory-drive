//! All the code in this file is human written
//! without any use of ai tools.
//!
//! TODO remove unwraps

mod archive;

use rayon::prelude::*;
use std::{path::Path, time::Instant};

use crate::archive::{
    create_compressed, create_thumbnail, load_archive_metadata, ArchiveMetadata, ImgMetadata, Mime,
};

#[tauri::command]
fn batch_operation(operation: String, archive_path: &Path, amount: usize, page: usize) -> u16 {
    let archive_metadata = load_archive_metadata(archive_path);
    let imgs: Vec<&ImgMetadata> = archive_metadata.flat_img_refs();

    let start = page * amount;
    let end = (start + amount).min(imgs.len());

    if start >= imgs.len() {
        return 5;
    }

    let chunk = &imgs[start..end];

    if operation == String::from("thumbnail") {
        chunk.par_iter().for_each(|img| match img.mime {
            Mime::Jpg | Mime::Png => {
                let _ = create_thumbnail(img);
            }
            _ => (),
        });
    }

    if operation == String::from("compress") {
        chunk.par_iter().for_each(|img| match img.mime {
            Mime::Jpg | Mime::Png => {
                let _ = create_compressed(img);
            }
            _ => (),
        });
    }

    return 0;
}

#[tauri::command]
fn load_archive(archive_path: &Path) -> ArchiveMetadata {
    let now = Instant::now();

    let archive_metadata = load_archive_metadata(archive_path);

    let elapsed = now.elapsed();
    println!("load_archive took: {:.2?}", elapsed);

    archive_metadata
}

#[tauri::command]
fn load_thumbnail(img_metadata: ImgMetadata) {
    let now = Instant::now();

    let _ = create_thumbnail(&img_metadata);

    let elapsed = now.elapsed();
    println!("load_thumbnail took: {:.2?}", elapsed);
}

#[tauri::command]
fn load_image(path: String) -> Result<Vec<u8>, String> {
    let now = Instant::now();
    let content = std::fs::read(path).map_err(|e| e.to_string());
    let elapsed = now.elapsed();
    println!("load_image took: {:.2?}", elapsed);
    content
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            load_archive,
            load_thumbnail,
            load_image,
            batch_operation
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
