//! All the code in this file is human written
//! without any use of ai tools.
//!
//! Parses archives with format
//! <archive-name>/2008/2008-01/img.jpg
//! <archive-name>/2008/2008-02/img.jpg
//! ...

#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Error};
use std::path::{Path, PathBuf};

use directories::ProjectDirs;

use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, ImageDecoder, ImageError, ImageReader};
use serde::Serialize;

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct ImgMetadata {
    full_img_path: PathBuf,
    thumb_img_path: PathBuf,
    img_name: String,
    mime: Mime,
    year: u16,
    month: u16,
}

#[derive(Serialize)]
pub struct ArchiveLeafMetadata {
    imgs: Vec<ImgMetadata>,
    total_imgs: u32,
    total_vids: u32,
}

type ArchiveLeafMap = HashMap<u16, ArchiveLeafMetadata>;

impl ArchiveLeafMetadata {
    fn new(mut imgs: Vec<ImgMetadata>) -> Self {
        let total_imgs = imgs.len() as u32;
        imgs.shrink_to_fit();

        ArchiveLeafMetadata {
            imgs,
            total_imgs,
            total_vids: 0,
        }
    }
}

#[derive(Serialize)]
pub struct ArchiveYearMetadata {
    year_months: ArchiveLeafMap,
    total_imgs: u32,
    total_vids: u32,
}

impl ArchiveYearMetadata {
    fn new(year_months: ArchiveLeafMap) -> Self {
        let total_imgs = year_months.iter().fold(0, |acc, x| acc + x.1.total_imgs);

        ArchiveYearMetadata {
            year_months,
            total_imgs,
            total_vids: 0,
        }
    }
}

#[derive(Serialize)]
pub struct ArchiveMetadata {
    years: HashMap<u16, ArchiveYearMetadata>,
    total_imgs: u32,
    total_vids: u32,
}

pub struct ArchiveDirectories {
    pub month_directories: Vec<PathBuf>,
    pub other_directories: Option<Vec<PathBuf>>,
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

fn process_img_thumbnail(img_metadata: &ImgMetadata) {
    match img_metadata.mime {
        Mime::Jpg => {
            if fs::exists(&img_metadata.thumb_img_path).unwrap() {
                return;
            }

            let mut decoder = ImageReader::open(&img_metadata.full_img_path)
                .unwrap()
                .into_decoder()
                .unwrap();
            let orientation = decoder.orientation().unwrap();
            let mut img = DynamicImage::from_decoder(decoder).unwrap();
            img.apply_orientation(orientation);

            let img = img.resize(350, 350, FilterType::Nearest);

            let thumb_parent = img_metadata.thumb_img_path.parent().unwrap();
            if !fs::exists(&thumb_parent).unwrap() {
                fs::create_dir_all(thumb_parent).unwrap();
            }

            let file = File::create(&img_metadata.thumb_img_path).unwrap();
            let mut writer = BufWriter::new(file);

            let encoder = JpegEncoder::new_with_quality(&mut writer, 25);
            img.write_with_encoder(encoder).unwrap();
        }
        _ => todo!(),
    }
}

pub fn create_thumbnail(img_metadata: &ImgMetadata) -> Result<(), ImageError> {
    let thumb_parent = img_metadata.thumb_img_path.parent().unwrap();

    if !fs::exists(&thumb_parent)? {
        fs::create_dir_all(thumb_parent)?;
    }

    match mime_to_filetype(&img_metadata.mime) {
        FileType::Image => process_img_thumbnail(img_metadata),
        FileType::Video => todo!(),
        FileType::Other => todo!(),
    }

    Ok(())
}

// Returns each leaf folder in the archive
pub fn analyse_archive(archive_path: &Path) -> Result<ArchiveDirectories, Error> {
    let mut months = Vec::new();

    for year_directory in fs::read_dir(archive_path)? {
        let path = year_directory?.path();

        for month_directory in fs::read_dir(&path)? {
            let month_path = month_directory?.path();

            months.push(month_path);
        }
    }

    let analysis = ArchiveDirectories {
        month_directories: months,
        // TODO support non-timestamped folders
        other_directories: None,
    };

    Ok(analysis)
}

pub fn load_leaf_directory_file_metadatas(
    dir_path: &Path,
    year: u16,
    month: u16,
) -> Result<ArchiveLeafMetadata, Error> {
    let mut dir_data: Vec<ImgMetadata> = Vec::new();
    let proj_dirs = ProjectDirs::from("com", "gcholette", "Memory Drive").unwrap();

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
            let mime = detect_mime(&file_path);

            let memory_drive_img: ImgMetadata = ImgMetadata {
                full_img_path: full_path.to_path_buf(),
                thumb_img_path: thumb_img_path.to_path_buf(),
                img_name: file_name.to_string_lossy().into_owned(),
                year,
                month,
                mime,
            };

            dir_data.push(memory_drive_img);
        }
    }

    Ok(ArchiveLeafMetadata::new(dir_data))
}

fn load_archive_metadata(archive_path: &Path) -> ArchiveMetadata {
    let ArchiveDirectories {
        month_directories,
        other_directories: _,
    } = analyse_archive(archive_path).unwrap();
    let mut archive_leafs: ArchiveLeafMap = HashMap::new();
    let mut archive_years: HashMap<u16, ArchiveLeafMap> = HashMap::new();

    for m in month_directories {
        let (year, month) = match m
            .file_name()
            .and_then(|x| x.to_str())
            .and_then(|x| x.split_once('-'))
        {
            Some(x) => (x.0.parse().unwrap_or(0), x.1.parse().unwrap_or(1)),
            None => (0, 0),
        };

        let mut imgs = load_leaf_directory_file_metadatas(&m, year, month).unwrap();
        // archive_years.insert(String::from(format!("{}", year)), imgs);
        if !archive_years.contains_key(&year) {
            let new_year = HashMap::new();
            new_year.insert(month, imgs);
            archive_years.insert(year, new_year);

        }
        // archive_leafs.insert(String::from(format!("{}", year)), imgs);
    }

    all_imgs_metadata
}
