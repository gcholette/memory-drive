use std::fs;
use std::fs::File;
use std::io::{BufWriter, Error};
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{thread, time};

use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, ImageDecoder, ImageError, ImageReader};

enum Mime {
    Jpg,
    Png,
    Heic,
    Mp4,
    Mov,
    Other,
}

enum FileType {
    Image,
    Video,
    Other,
}

struct ImgMetadata {
    full_img_path: PathBuf,
    thumb_img_path: PathBuf,
    img_name: String,
    mime: Mime,
    year: u16,
    month: u16,
}

struct ArchiveAnalysis {
    month_directories: Vec<PathBuf>,
    other_directories: Option<Vec<PathBuf>>
}

const DEFAULT_ARCHIVE_PATH: &str = "/home/gcholette/Pictures/mdrive_test/";

fn load_img(file_path: &str) -> Vec<u8> {
    let image_data: Vec<u8> = fs::read(file_path).expect("Should have been able to read the file");

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
        Mime::Other => FileType::Other,
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
        }
        _ => todo!(),
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

// Returns each leaf folder in the archive
fn analyse_archive(archive_path: &str) -> Result<ArchiveAnalysis, Error> {
    let mut months = Vec::new();

    for year_directory in fs::read_dir(archive_path)? {
        let path = year_directory?.path();

        for month_directory in fs::read_dir(&path)? {
            let month_path = month_directory?.path();

            months.push(month_path);
        }
    }

    let analysis = ArchiveAnalysis {
        month_directories: months,
        // TODO support non-timestamped folders
        other_directories: None
    };

    Ok(analysis)
}

fn load_leaf_directory_file_metadatas(dir_path: &Path, archive_path: &str) -> Result<Vec<ImgMetadata>, Error> {
    let mut dir_data: Vec<ImgMetadata> = Vec::new();
    let archive_path = Path::new(archive_path);

    // TODO currently assumes that all leaf folders will have the correct name format YYYY-MM
    // other folders should have year/month at 0
    for file in fs::read_dir(dir_path)? {
        let file_path: String = file?.path().to_string_lossy().into_owned();
        let full_path = Path::new(dir_path);
        let file_name = full_path.file_name().unwrap().to_string_lossy();

        // TODO figure out how to save thumbnails
        let thumb_img_path = archive_path.join(".memory-drive/");

        let year = file_name
            .chars()
            .take(4)
            .collect::<String>()
            .parse::<u16>()
            .unwrap();

        let month = file_name
            .chars()
            .skip(5)
            .take(2)
            .collect::<String>()
            .parse::<u16>()
            .unwrap();

        let mime = detect_mime(&file_path);

        let memory_drive_img: ImgMetadata = ImgMetadata { 
            full_img_path: full_path.to_path_buf(), 
            thumb_img_path, 
            img_name: file_name.into_owned(), 
            year, 
            month,
            mime
        };

        dir_data.push(memory_drive_img);

    }

    Ok(dir_data)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let ArchiveAnalysis { month_directories, other_directories: _ } = analyse_archive(DEFAULT_ARCHIVE_PATH).unwrap();
    let mut all_imgs_metadata:Vec<ImgMetadata> = Vec::new();

    let now = Instant::now();
    for m in month_directories {
        let mut imgs = load_leaf_directory_file_metadatas(&m, DEFAULT_ARCHIVE_PATH).unwrap();
        all_imgs_metadata.append(&mut imgs);
    }

    let elapsed = now.elapsed();
    println!("Loading metadata took: {:.2?}", elapsed);
    println!("Total vector length {:?}", all_imgs_metadata.len());

    let sleep_time = time::Duration::from_millis(100000);
    thread::sleep(sleep_time);

    // let _ = create_thumbnail("");

    //tauri::Builder::default()
        //.plugin(tauri_plugin_opener::init())
        // .invoke_handler(tauri::generate_handler![analyse_archive])
        //.run(tauri::generate_context!())
        //.expect("error while running tauri application");
}
