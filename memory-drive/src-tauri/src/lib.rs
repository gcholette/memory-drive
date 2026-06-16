use std::fs;
use std::fs::File;
use std::io::BufWriter;

use image::{ImageBuffer, ImageReader, ImageResult, DynamicImage, save_buffer, ExtendedColorType};
use image::codecs::jpeg::{JpegDecoder, JpegEncoder};

use std::any::{type_name, type_name_of_val};

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

fn process_img_thumbnail(img: &DynamicImage, mime: Mime) -> Result<BufWriter<File>, ()> {
    match mime {
        Mime::Jpg => {
            // let mut writer = vec![];
            let file = File::create("/home/gcholette/Pictures/output.jpg");
            let writer = BufWriter::new(file);

            let encoder = JpegEncoder::new_with_quality(&mut writer, 6);
            let result = img.write_with_encoder(encoder).unwrap();

            let height = img.height();
            let width = img.width();

            // let img2 = ImageBuffer::from_vec(width, height, writer);

            println!("{}", height);
            println!("{}", width);



            return Ok(writer)
        },
        _ => todo!()
    }

}

fn create_thumbnail(file_path: &str) -> Result<(), ()> {
    let img = image::open("/home/gcholette/Pictures/100_1146.JPG").unwrap();

    let thumb = process_img_thumbnail(&img, Mime::Jpg).unwrap();

    // save_buffer("/home/gcholette/Pictures/out1.jpg", &thumb, 2856,2142, ExtendedColorType::Rgb16);

    Ok(())
}

// #[tauri::command]
// fn load_imgs_in_path(folder_path: &str) -> Vec<ImageReader> {
//     let dir: fs::ReadDir  = fs::read_dir(folder_path)
//         .expect("Couldn't open the folder");

//     let mut img_data: Vec<ImageReader> = vec![];

//     for result in dir {
//         match result {
//             Err(e) => println!("Error while loading dir: {}", e),
//             Ok(v) => {
//                 let path = v.path();
//                 let img = image::open(&path.to_string_lossy()).unwrap();
//                 img_data.push(img)
//                 // img_data.push(load_img(&path.to_string_lossy()))
//             }
//         }
//     }

//     img_data
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    create_thumbnail("");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        //.invoke_handler(tauri::generate_handler![load_imgs_in_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
