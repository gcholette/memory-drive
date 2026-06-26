//! All the code in this file is human written
//! without any use of ai tools.
//! 
//! TODO remove unwraps

mod archive;

use std::{path::Path, time::Instant};
use rayon::prelude::*;

use crate::archive::{ArchiveMetadata, ImgMetadata, Mime, create_compressed, create_thumbnail, load_archive_metadata};

#[tauri::command]
fn compress_batch(archive_path: &Path, amount: usize, page: usize) -> u16{
    let archive_metadata = load_archive_metadata(archive_path);
    let imgs: Vec<&ImgMetadata> = archive_metadata.flat_img_refs();

    let start = page * amount;
    let end = (start + amount).min(imgs.len());

    if start >= imgs.len() {
        return 5;
    }

    let chunk = &imgs[start..end];

    // let now = Instant::now();

    chunk.par_iter().for_each(|img| {
        match img.mime {
            Mime::Jpg | Mime::Png => {
                let _ = create_compressed(img);
            },
            _ => ()
        }
    });

    //println!("Compression batch completed in: {:.2?}", now.elapsed());

    return 0
}

#[tauri::command]
fn cache_all_thumbnails(archive_path: &Path) -> u16{
    let archive_metadata = load_archive_metadata(archive_path);

    let imgs: Vec<&ImgMetadata> = archive_metadata.flat_img_refs();

    let now = Instant::now();

    imgs.par_iter().for_each(|img| {
        match img.mime {
            Mime::Jpg => {
                let _ = create_thumbnail(img);
            },
            _ => ()
        }
    });

    let elapsed = now.elapsed();
    println!("Thumbnail created in: {:.2?}", elapsed);

    return 0
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
        .invoke_handler(tauri::generate_handler![load_archive, load_thumbnail, load_image, cache_all_thumbnails, compress_batch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
