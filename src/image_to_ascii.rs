use crate::image_processor::ImageProcessor;

// const ASCII_CHARS: &str = "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
const ASCII_CHARS: &str = "█▓▒░┍┑┕┙━┃┎┒┖┚│┌┐└┘╭╮╰╯╱╲╳●○◯◔◕◐◑◒◓◍◎◉✺✹✸✷✶✵✴✳✲✱✰✯✭✬✫✪✩✧✦✥✤✣✢✡";
// const ASCII_CHARS: &str = "░▒▓█■□▢▣▤▥▦▧▨▩▪▫▬▭▮▯▰▱▲△▴▵▶▷▸▹►▻▼▽▾▿◀◁";

pub type AsciiData = Vec<Vec<char>>;

pub struct ImageToAscii {
    image_processor: ImageProcessor,
    scale_factor: f32,
    max_width: Option<usize>,
}

impl ImageToAscii {
    pub fn new(image_path: String) -> Self {
        let image_processor = ImageProcessor::new(image_path);
        ImageToAscii {
            image_processor,
            scale_factor: 1.0,
            max_width: None,
        }
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale_factor = scale;
        self
    }

    pub fn with_max_width(mut self, width: usize) -> Self {
        self.max_width = Some(width);
        self
    }

    fn convert_to_ascii(&self, brightness: u32) -> char {
        let ascii_index =
            ((ASCII_CHARS.chars().count() - 1) as f64 * (brightness as f64 / 255.0)) as usize;

        // println!("brightness: {} | ascii_index: {}", brightness, ascii_index);
        let ascii_value = ASCII_CHARS.chars().nth(ascii_index).unwrap();
        ascii_value
    }

    fn set_ascii_matrix(&self, brightness_matrix: Vec<Vec<i32>>) -> AsciiData {
        let mut ascii_matrix = Vec::with_capacity(brightness_matrix.len());

        for row in brightness_matrix {
            let mut ascii_row = Vec::with_capacity(row.len());

            for cell in row {
                ascii_row.push(self.convert_to_ascii(cell as u32));
            }

            ascii_matrix.push(ascii_row);
        }

        ascii_matrix
    }

    pub fn print_ascii_matrix(&self) {
        let brightness_matrix = self.image_processor.get_brightness_matrix();
        let pixels = self.set_ascii_matrix(brightness_matrix);

        std::fs::create_dir_all("output").expect("Failed to create output directory");

        for i in 0..pixels.len() {
            let mut row: String = "".to_string();

            print!("row_{:04} ", i);
            row += format!("row_{:04} ", i).as_str();
            for j in 0..pixels[0].len() {
                let val = format!("{}", pixels[i][j]);
                row += &val;
                print!("{}", val);

                // let filename = format!("{}", pixels[i][j]);
                // let filepath = format!("output/{}.txt", filename);
                // let _ = File::create(&filepath).expect("Failed to create file");
            }

            // let filename = row.to_string();
            // let filepath = format!("output/.{:04}{}.txt", i, filename);
            // let _ = File::create(&filepath)
            //     .unwrap_or_else(|_| panic!("Failed to create file: {}", filepath));

            // row = "".to_string();
            println!();
        }
    }
}
