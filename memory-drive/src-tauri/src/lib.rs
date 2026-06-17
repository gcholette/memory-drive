use std::fs;
use std::fs::File;
use std::io::{BufWriter, Error};
use std::time::Instant;

use image::{ImageDecoder, ImageError, ImageReader, DynamicImage};
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;

enum Mime {
    Jpg,
    Png,
    Heic,
    Mp4,
    Mov,
    Other
}

enum FileType {
    Image,
    Video,
    Other
}

const DEFAULT_ARCHIVE_PATH: &str = "/home/gcholette/Pictures/mdrive_test/";

fn load_img(file_path: &str) -> Vec<u8> {
    let image_data:Vec<u8> = fs::read(file_path)
    .expect("Should have been able to read the file");

    image_data
}

fn detect_mime(file_path: &str) -> Mime {
    if file_path.to_ascii_lowercase().contains(".jpg") {
        return Mime::Jpg;
    }

    Mime::Other
}

fn mime_to_filetype(mime: &Mime) -> FileType {
    match mime {
        Mime::Jpg | Mime::Png | Mime::Heic => FileType::Image,
        Mime::Mp4 | Mime::Mov => FileType::Video,
        Mime::Other => FileType::Other
    }
}

fn process_img_thumbnail(img: &DynamicImage, mime: Mime) {
    match mime {
        Mime::Jpg => {
            let img = img.resize(350, 350, FilterType::Nearest);
            
            let file = File::create("/home/gcholette/Pictures/mdrive_output.jpg").unwrap();
            let mut writer = BufWriter::new(file);

            let encoder = JpegEncoder::new_with_quality(&mut writer, 25);
            let result = img.write_with_encoder(encoder).unwrap();
        },
        _ => todo!()
    }

}

fn create_thumbnail(img_path: &str) -> Result<(), ImageError> {

    let now = Instant::now();

    let mut decoder = ImageReader::open(img_path)?.into_decoder()?;
    let orientation = decoder.orientation()?;
    let mut img = DynamicImage::from_decoder(decoder)?;
    img.apply_orientation(orientation);

    process_img_thumbnail(&img, Mime::Jpg);

    let elapsed = now.elapsed();
    println!("Loading: {:.2?}", elapsed);

    Ok(())
}

// #[tauri::command]
fn analyse_archive(archive_path: &str) -> Result<Vec<String>, Error> {
    let mut months = Vec::new();

    for year_directory in fs::read_dir(archive_path)? {
        let path = year_directory?.path();

        for month_directory in fs::read_dir(&path)? {
            let month_path = month_directory?
                .path()
                .to_string_lossy()
                .into_owned();

            months.push(month_path);
        }
    }

    Ok(months)
}

fn list_archive_dir(dir_path: &str) -> Result<Vec<String>, Error> {
    let mut dir_data = Vec::new();

    for file in fs::read_dir(dir_path)? {
        let file_path: String = file?.path().to_string_lossy().into_owned();
        let mime = detect_mime(&file_path);
        match mime {
            Mime::Jpg => dir_data.push(file_path),
            _ => ()
        }
    }

    Ok(dir_data)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let months = analyse_archive(DEFAULT_ARCHIVE_PATH).unwrap();
    for m in months {
        println!("{}", m);
    }

    // let _ = create_thumbnail("");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // .invoke_handler(tauri::generate_handler![analyse_archive])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
