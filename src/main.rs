use clap::Parser;
use std::{fs, io::Write, os::windows::process};

use image::EncodableLayout;

const CHAR_SET_1: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. "; //"@%#*+=-:. ";
const CHAR_SET_2: &str = "@%#*+=-:. ";

/// Image to ascii text convertor
#[derive(Parser, Debug)]
struct Args {
    /// Path to the input file
    #[clap(short, long)]
    r#in: String,

    /// Path to the output (txt) file. If none specified, the output is dumped into stdout. [optional]
    #[clap(short, long)]
    out: Option<String>,

    /// The ammount of horizontal pixels that get compressed into one character of the result (the greater the value, the smaller the image)
    #[clap(long, default_value_t = 15)]
    compression: u8,

    /// The used set of characters for image generation (1 = more characters, better detail, less contrast; 2 = less characters, worse detail, more contrast)
    #[clap(long, default_value_t = 1)]
    charset: u8,
}
fn main() {
    let args = Args::parse();

    println!("Processing '{}'...\n", args.r#in);

    let charset = match args.charset {
        1 => CHAR_SET_1,
        2 => CHAR_SET_2,
        _ => return eprintln!("Err: Invalid charset '{}'.", args.charset),
    };
    let chars: Vec<char> = charset.chars().collect();

    // Value between chars on the darkness ramp
    let char_step = 255_f32 / ((chars.len() - 1) as f32);

    let img = image::open(args.r#in).unwrap().to_rgba8();

    let width = img.width() as usize;
    let height = img.height() as usize;

    let sample_frequency = args.compression as usize;
    let img_data = img.as_bytes();

    let mut output = String::new();

    // Position on the current row
    let mut row_cursor = 0;

    // Index of the current row
    let mut row = 0;

    while (row_cursor * row) < img_data.len() - 1 {
        // List of pixels that will be merged
        let mut pixels_to_sample = vec![];
        // Loop through all rows that will get merged
        for r in row..(row + (sample_frequency * 2)) {
            // Calculate absolute index
            let sample_start = (r * width + row_cursor) * 4;

            // Current cursor of the row
            let mut rc = sample_start;
            // Loop over the row
            while rc < (sample_start + (sample_frequency * 4)) {
                if rc + 4 > img_data.len() {
                    break;
                }

                // Add up values of R,G,B,A
                let total_value = img_data[rc..(rc + 4)].iter().fold(0u16, |mut sum, &val| {
                    sum += val as u16;
                    sum
                });
                // Calculate the average value and push in the array
                pixels_to_sample.push(total_value / 4);
                rc += 4;
            }
        }

        // Add up all the pixel average
        let total = pixels_to_sample.iter().fold(0u32, |mut sum, &val| {
            sum += val as u32;
            sum
        });

        // Calculate the average value
        let average = total as f32 / pixels_to_sample.len() as f32;

        // Calculate what character should be used
        let ch = chars[((average / char_step) as f32).round() as usize];
        output.push(ch);

        // Handle end of row and jump to next one
        if width - row_cursor <= sample_frequency {
            row_cursor = 0;
            if row + sample_frequency * 2 >= height {
                break;
            }
            row += sample_frequency * 2;
            output.push('\n');
            continue;
        }
        row_cursor += sample_frequency;
    }

    // Output the result
    println!("Finished\n");
    if let Some(out) = args.out {
        fs::write(&out, output).unwrap();
        println!("Saved to '{}'", &out);
    } else {
        println!("{:-^1$}", "Output", width / &sample_frequency);
        std::io::stdout().write_all(output.as_bytes()).unwrap();
        println!("\n{}", "-".repeat(width / sample_frequency));
    }
}
