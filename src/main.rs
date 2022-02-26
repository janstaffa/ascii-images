use clap::Parser;
use std::fs;

use image::EncodableLayout;

const CHARS: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. "; //"@%#*+=-:. ";

/// Image to ascii text convertor
#[derive(Parser, Debug)]
struct Args {
    /// Path to the input file
    #[clap(short, long)]
    r#in: String,

    /// Path to the output (txt) file
    #[clap(short, long)]
    out: String,

    /// The quality of the result image (in % relative to the original)
    #[clap(short, long, default_value_t = 100)]
    quality: u8,
}
fn main() {
    let args = Args::parse();

    println!("Converting...");
    let chars: Vec<char> = CHARS.chars().collect();
    let char_step = 255_f32 / ((chars.len() - 1) as f32);
    let img = image::open(args.r#in).unwrap().to_rgba8();

    let (width, height) = img.dimensions();

    let width = width as usize;
    let height = height as usize;

    let img_data = img.as_bytes();

    let mut output = String::new();

    let mut row_cursor = 0;
    let mut col = 0;

    while (row_cursor * col) < img_data.len() - 1 {
        let top_idx = ((col * width) + row_cursor) * 4;
        let bottom_idx = (((col + 1) * width) + row_cursor) * 4;

        let top_pixel = img_data[top_idx..top_idx + 4]
            .iter()
            .fold(0u16, |mut sum, &val| {
                sum += val as u16;
                sum
            });
        let bottom_pixel =
            &img_data[bottom_idx..bottom_idx + 4]
                .iter()
                .fold(0u16, |mut sum, &val| {
                    sum += val as u16;
                    sum
                });

        // Calculate what character should be used
        let average = (top_pixel + bottom_pixel) as f32 / 8_f32;

        let ch = chars[((average / char_step) as f32).round() as usize];
        output.push(ch);

        if row_cursor == width - 1 {
            row_cursor = 0;
            if col + 2 >= height {
                break;
            }
            col += 2;
            output.push('\n');
            continue;
        }
        row_cursor += 1;
    }

    println!("\nDone");
    fs::write(args.out, output).unwrap();
}
