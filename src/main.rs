#![allow(clippy::or_fun_call)]

use clap::Parser;
use phf::{phf_map, Map};
use std::{
    fs::File,
    io::{Write, BufReader}, error::Error,
};

use image::{ImageBuffer, Luma, io::Reader};

#[derive(Parser)]
#[clap(
    name = "Ascii Converter",
    author = "FeistyKit <eeveebeevee33@gmail.com>"
)]
struct Args {

    #[clap(short, long, default_value_t = 8)]
    /// The amount of pixels packed into one character, width wise.
    ///
    /// Higher values mean that the image will be thinner. For most fonts,
    /// results look best when this is about 0.5x width_compression.
    width_compression: u32,

    #[clap(short, long, default_value_t = 16)]
    /// The amount of pixels packed into one character, height wise.
    ///
    /// Higher values mean that the image will be shorter. For most fonts,
    /// results look best when this is about 2x width_compression.
    height_compression: u32,

    input_filepath: String,

    #[clap(short, long, default_value = "output.txt")]
    /// The filepath to output to.
    output_filepath: String,

    #[clap(short, long)]
    /// Whether or not to use a dark-mode colour scheme.
    dark_mode: bool,
}

static DARKNESS_MAP: Map<u8, char> = phf_map! {
    0u8 => ' ',
    1u8 => '.',
    2u8 => ':',
    3u8 => '-',
    4u8 => '=',
    5u8 => '+',
    6u8 => '*',
    7u8 => '#',
    8u8 => '%',
    9u8 => '@',
};

type MaybeError = Result<(), Box<dyn Error>>;

fn main() -> MaybeError {
    let args = Args::parse();
    let initial_image = open_file(&args.input_filepath)?;
    let details = prepare_image_details(&initial_image, &args);
    let output_string = pixel_each(initial_image, details);
    save(output_string, &args.output_filepath)
}

fn save(output_string: String, path: &str) -> MaybeError {
    let mut b = File::create(path)?;
    b.write_all(output_string.as_bytes())?;
    println!("Finished!");
    Ok(())
}

fn prepare_image_details(initial_image: &ImageBuffer<Luma<u8>, Vec<u8>>, args: &Args) -> ImageDetails {
    let (w, h) = initial_image.dimensions();
    ImageDetails {
        w,
        h,
        split_w: args.width_compression,
        split_h: args.height_compression,
        dark: args.dark_mode,
    }
}

//goes through pixels to determine the darkness of each
fn pixel_each(initial_image: ImageBuffer<Luma<u8>, Vec<u8>>, details: ImageDetails) -> String {
    let mut output_string = String::new();
    for q in 0..(details.h / details.split_h as u32) {
        for i in 0..(details.w / details.split_w as u32) {
            //the image is split into "chunks" so that it can be scaled
            let mut temp_vec = vec![]; //vector to average the pixels of the chunk to get the chunk's darkness value
            for a in i * details.split_w..(i + 1) * details.split_w {
                for b in q * details.split_h..(q + 1) * details.split_h {
                    temp_vec.push(initial_image.get_pixel(a, b).0[0]);
                }
            }
            let total: u64 = temp_vec.iter().map(|f| *f as u64).sum();
            let count: u64 = temp_vec.len() as u64;
            //gets average darkness of the vector
            output_string.push(get_char((total / count) as u8, details.dark));
        }
        output_string.push('\n');
    }
    output_string
}
//gets char from the map depending on darkness value. the math is so that the value can be changed to 10 options.
fn get_char(i: u8, dark: bool) -> char {
    //For users who use a light background
    match dark {
        true => DARKNESS_MAP[&((i as f64 / 256.0 * 10.0) as u8)],
        false => DARKNESS_MAP[&(9 - (i as f64 / 256.0 * 10.0) as u8)],
    }

}

fn open_file(p: &str) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
    let b = Reader::new(BufReader::new(File::open(p)?)).with_guessed_format()?.decode()?.into_luma8();
    Ok(b)
}

struct ImageDetails {
    w: u32,
    h: u32,
    split_w: u32,
    split_h: u32,
    dark: bool,
}
