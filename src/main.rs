#![allow(clippy::or_fun_call)]
use phf::{phf_map, Map};
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Write,
};

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
fn main() {
    let (map, string_prompt) = prepare_map();
    let initial_image = open_from_map(&map, &string_prompt);
    let details = prepare_image_details(&initial_image);
    let output_string = pixel_each(initial_image, details);
    save(output_string);
    pause();
}

fn save(output_string: String) {
    let mut b = File::create("output.txt").unwrap();
    b.write_all(output_string.as_bytes()).unwrap();
    println!("Finished!");
}

fn prepare_image_details(initial_image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageDetails {
    let (w, h) = initial_image.dimensions();
    let split_h = input("Image height step (default 16): ")
        .unwrap_or("16".to_string())
        .trim()
        .parse::<u32>()
        .unwrap_or(16);
    let split_w = input("Image width step (default 8): ")
        .unwrap_or("8".to_string())
        .trim()
        .parse::<u32>()
        .unwrap_or(8);
    ImageDetails {
        w,
        h,
        split_w,
        split_h,
    }
}

fn prepare_map() -> (BTreeMap<usize, String>, String) {
    let map = file_map();
    let mut string_prompt =
        "Input the number that corresponds to the file that you want to convert: ".to_string();
    for (k, v) in map.iter() {
        string_prompt.push_str(&format!("\n{}: {}", k, v));
    }
    (map, string_prompt)
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
            output_string.push(get_char((total / count) as u8));
        }
        output_string.push('\n');
    }
    output_string
}
fn input(m: &str) -> Result<String, std::io::Error> {
    println!("{}", m);
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)?;
    Ok(s)
}
//gets char from the map depending on darkness value. the math is so that the value can be changed to 10 options.
fn get_char(i: u8) -> char {
    //For users who use a light background; will add an option to switch depending on text viewer.
    DARKNESS_MAP[&(9 - (i as f64 / 256.0 * 10.0) as u8)]
}

fn open_file(p: &str) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
    let b = open(p)?.into_luma8();
    Ok(b)
}
//creates map to get user input
fn file_map() -> BTreeMap<usize, String> {
    let b = fs::read_dir("./").expect("Could not read directory!");
    let mut x = 1;
    let mut map = BTreeMap::new();
    for i in b {
        let h = i.unwrap().file_name();
        let q = h.to_str().unwrap();
        if q.contains(".png") || q.contains(".jpg") {
            map.insert(x, q.to_string());
            x += 1;
        }
    }
    map
}
fn open_from_map(
    map: &BTreeMap<usize, String>,
    string_prompt: &str,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    match map.get(
        &input(string_prompt)
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap_or(0),
    ) {
        Some(s) => open_file(s).unwrap(),
        None => open_from_map(map, "That is not a valid entry!"),
    }
}
#[allow(unused_imports)]
fn pause() {
    use std::io::{stdin, stdout, Read, Write};
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}

struct ImageDetails {
    w: u32,
    h: u32,
    split_w: u32,
    split_h: u32,
}
