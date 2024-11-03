// use ffmpeg_next as ffmpeg;
use std::{fs, io::Write, process::Command, thread, time::Duration};

use image_to_ascii::ImageToAscii;

pub mod error;
pub mod image_processor;
pub mod image_to_ascii;

const CLEAR_SCREEN: &str = "\x1B[2J\x1B[1;1H";

fn main() {
    // ffmpeg::init().expect("Failed to initialize FFmpeg");
    //
    // let video_path = "video.mp4";
    // let mut ictx = ffmpeg::init().unwrap();
    // let input = ictx
    //     .streams()
    //     .best(ffmpeg::media::Type::Video)
    //     .ok_or("Could not find video stream")?;
    // let video_stream_index = input.index();
    //
    // if let Ok(mut ictx) = input(&env::args().nth(1).expect("Cannot open file.")) {}

    // let image_paths = ["art.png", "art_two.png"];
    // let converters: Vec<ImageToAscii> = image_paths
    //     .iter()
    //     .map(|&path| ImageToAscii::new(path.to_string()))
    //     .collect();

    let image_paths = (1..=270).collect::<Vec<i32>>();
    let converters: Vec<ImageToAscii> = image_paths
        .iter()
        .map(|&path| {
            let filename = format!("fire/{:04}.png", path); // Creates: frame_0001.jpg, frame_0002.jpg, etc.
            ImageToAscii::new(filename)
        })
        .collect();

    // loop {
    for converter in &converters {
        print!("{}", CLEAR_SCREEN);
        // for entry in fs::read_dir("output").expect("Failed to read output directory") {
        //     let entry = entry.expect("Failed to access directory entry");
        //     fs::remove_file(entry.path()).expect("Failed to delete file");
        // }

        converter.print_ascii_matrix();

        std::io::stdout().flush().unwrap();
        // refresh_finder();
        thread::sleep(Duration::from_millis(100));
    }

    fn refresh_finder() {
        Command::new("osascript")
            .arg("-e")
            .arg("tell application \"Finder\" to update front window")
            .output()
            .expect("Failed to refresh Finder");
    }
    // }

    // let image_to_ascii = ImageToAscii::new(image_path);
    // image_to_ascii.print_ascii_matrix();
}
