//! All the code in this file is human written
//! without any use of ai tools.

#![allow(dead_code)]

use std::fs;
use std::fs::File;
use std::io::{BufWriter, Error};
use std::path::{Path, PathBuf};

use directories::ProjectDirs;

use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, ImageDecoder, ImageError, ImageReader};

#[derive(Debug)]
pub enum Mime {
    Jpg,
    Png,
    Heic,
    Mp4,
    Mov,
    Other,
}

pub enum FileType {
    Image,
    Video,
    Other,
}

#[derive(Debug)]
pub struct ImgMetadata {
    full_img_path: PathBuf,
    thumb_img_path: PathBuf,
    img_name: String,
    mime: Mime,
    year: u16,
    month: u16,
}

pub struct ArchiveAnalysis {
    pub month_directories: Vec<PathBuf>,
    pub other_directories: Option<Vec<PathBuf>>
}

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

fn process_img_thumbnail(img: &DynamicImage, img_metadata: &ImgMetadata) {
    match img_metadata.mime {
        Mime::Jpg => {
            let img = img.resize(350, 350, FilterType::Nearest);

            let file = File::create("/home/gcholette/Pictures/mdrive_output.jpg").unwrap();
            let mut writer = BufWriter::new(file);

            let encoder = JpegEncoder::new_with_quality(&mut writer, 25);
            img.write_with_encoder(encoder).unwrap();
        }
        _ => todo!(),
    }
}

fn create_thumbnail(img_metadata: &ImgMetadata) -> Result<(), ImageError> {
    let mut decoder = ImageReader::open(&img_metadata.full_img_path)?.into_decoder()?;
    let orientation = decoder.orientation()?;
    let mut img = DynamicImage::from_decoder(decoder)?;
    img.apply_orientation(orientation);

    process_img_thumbnail(&img, img_metadata);

    Ok(())
}

// Returns each leaf folder in the archive
pub fn analyse_archive(archive_path: &str) -> Result<ArchiveAnalysis, Error> {
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

pub fn load_leaf_directory_file_metadatas(dir_path: &Path) -> Result<Vec<ImgMetadata>, Error> {
    let mut dir_data: Vec<ImgMetadata> = Vec::new();
    let proj_dirs = ProjectDirs::from("com", "gcholette",  "Memory Drive").unwrap();

    // TODO currently assumes that all leaf folders will have the correct name format YYYY-MM
    // other folders should have year/month at 0
    for file in fs::read_dir(dir_path)? {
        let file_path: String = file?.path().to_string_lossy().into_owned();
        let full_path = Path::new(&file_path);
        let parent_path = full_path.parent().expect("couldn't infer parent path.");
        let year_txt = parent_path.file_name().unwrap();
        let file_name = full_path.file_name().unwrap();

        let thumb_img_path = proj_dirs
            .cache_dir()
            .join(year_txt)
            .join(format!("thumb-{}", file_name.display()));

        if !full_path.is_dir() {
            let year = year_txt
                .to_string_lossy()
                .chars()
                .take(4)
                .collect::<String>()
                .parse::<u16>()
                .unwrap();

            let month = year_txt
                .to_string_lossy()
                .chars()
                .skip(5)
                .take(2)
                .collect::<String>()
                .parse::<u16>()
                .unwrap();

            let mime = detect_mime(&file_path);

            let memory_drive_img: ImgMetadata = ImgMetadata { 
                full_img_path: full_path.to_path_buf(), 
                thumb_img_path: thumb_img_path.to_path_buf(), 
                img_name: file_name.to_string_lossy().into_owned(), 
                year, 
                month,
                mime
            };

            dir_data.push(memory_drive_img);

        }



    }

    Ok(dir_data)
}
