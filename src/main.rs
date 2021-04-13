use phf::{phf_map, Map};
use std::{fs::File, io::Write};

use image::{open, ImageBuffer, Luma};
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
const SPLITW: u32 = 8;
const SPLITH: u32 = 16;
fn main() {
    let mut output_string = String::new();
    let initial_image = open_file(input("File name: ").unwrap().trim()).unwrap();
    let (w, h) = initial_image.dimensions();
    pixel_each(h, w, initial_image, &mut output_string);
    let mut b = File::create("output.txt").unwrap();
    b.write_all(output_string.as_bytes()).unwrap();
}

fn pixel_each(
    h: u32,
    w: u32,
    initial_image: ImageBuffer<Luma<u8>, Vec<u8>>,
    output_string: &mut String,
) {
    for q in 0..h / SPLITH as u32 {
        for i in 0..w / SPLITW as u32 {
            let mut temp_vec = vec![];
            for a in i * SPLITW..(i + 1) * SPLITW {
                for b in q * SPLITH..(q + 1) * SPLITH {
                    temp_vec.push(initial_image.get_pixel(a, b).0[0]);
                }
            }
            let total: u64 = temp_vec.iter().map(|f| *f as u64).sum();
            let count: u64 = temp_vec.len() as u64;
            output_string.push(get_char((total / count) as u8));
        }
        output_string.push('\n');
    }
}
fn input(m: &str) -> Result<String, std::io::Error> {
    println!("{}", m);
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)?;
    Ok(s)
}
fn get_char(i: u8) -> char {
    DARKNESS_MAP[&(9 - (i as f64 / 256.0 * 10.0) as u8)]
}
fn open_file(p: &str) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
    Ok(open(p)?.into_luma8())
}
