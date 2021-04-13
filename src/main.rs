#![allow(clippy::or_fun_call)]
use phf::{phf_map, Map};
use std::{
    collections::HashMap,
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
    let mut output_string = String::new();
    let (map, string_prompt) = prepare_map();
    let initial_image = open_from_map(&map, &string_prompt);

    let (w, h, split_h, split_w) = prepare_image_details(&initial_image);
    pixel_each(h, w, initial_image, &mut output_string, split_w, split_h);
    let mut b = File::create("output.txt").unwrap();
    b.write_all(output_string.as_bytes()).unwrap();
    println!("Finished!");
    pause();
}

fn prepare_image_details(initial_image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> (u32, u32, u32, u32) {
    let (w, h) = initial_image.dimensions();
    let split_h = input("Image height step (default 16): ")
        .unwrap_or("16".to_string())
        .parse::<u32>()
        .unwrap_or(16);
    let split_w = input("Image width step (default 8): ")
        .unwrap_or("8".to_string())
        .parse::<u32>()
        .unwrap_or(8);
    (w, h, split_h, split_w)
}

fn prepare_map() -> (HashMap<usize, String>, String) {
    let map = file_map();
    let mut string_prompt =
        "Input the number that corresponds to the file that you want to convert: ".to_string();
    let mut b: Vec<_> = map.iter().collect();
    b.sort_by(|a, b| a.0.cmp(b.0));
    for (k, v) in &b {
        string_prompt.push_str(&format!("\n{}: {}", k, v));
    }
    (map, string_prompt)
}

fn pixel_each(
    h: u32,
    w: u32,
    initial_image: ImageBuffer<Luma<u8>, Vec<u8>>,
    output_string: &mut String,
    split_w: u32,
    split_h: u32,
) {
    for q in 0..h / split_h as u32 {
        for i in 0..w / split_w as u32 {
            let mut temp_vec = vec![];
            for a in i * split_w..(i + 1) * split_w {
                for b in q * split_h..(q + 1) * split_h {
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
    let b = open(p)?.into_luma8();
    Ok(b)
}
/*
fn open_file_safe(p: &str) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    if !p.contains(".png") && !p.contains(".jpg") {
        return open_file_safe(
            &input("The only supported file types are .png and .jpg! Please enter a valid file:")
                .unwrap()
                .trim(),
        );
    }
    let b = open_file(p);
    match b {
        Err(_) => open_file_safe(
            &input("An error occurred, please enter again (maybe the file wasn't found?):")
                .unwrap()
                .trim(),
        ),
        Ok(s) => s,
    }
}
// Implement light/dark mode so that it is not inverted.

fn l_d_input(s: &str) -> String {
    match input(s) {
        Ok(f) => f,
        Err(_) => l_d_input("That is not valid!"),
    }
}
fn p_l_d_input(s: String) -> bool {}
*/
fn file_map() -> HashMap<usize, String> {
    let b = fs::read_dir("./").unwrap();
    let mut x = 1;
    let mut map = HashMap::new();
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
    map: &HashMap<usize, String>,
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
