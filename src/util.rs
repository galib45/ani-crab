use std::io::{stdin, stdout, Write};
use std::env::args;
use std::path::Path;
use std::error::Error;

const VERSION: &str = "0.1.0";
pub const COLOR_RED_BOLD: &str = "\x1b[1;31m";
pub const COLOR_GREEN_BOLD: &str = "\x1b[1;32m";
pub const COLOR_CYAN_BOLD: &str = "\x1b[1;36m";
pub const COLOR_RESET: &str = "\x1b[0m";

#[derive(Debug)]
pub struct Config {
    pub has_task: bool,
    pub download: bool,
    pub quality: u32,
    pub range_start: u32,
    pub range_end: u32,
}

pub fn parse_args() -> Result<Config, Box<dyn Error>> {
    let mut config = Config {
        has_task: true,
        download: false,
        quality: 720,
        range_start: 0,
        range_end: 0
    };
    let mut arg;
    let argv: Vec<String> = args().collect();
    let argc = argv.len();
    let program_name = Path::new(&argv[0])
        .file_stem().unwrap().to_str().unwrap();
    let mut counter = 1;
    loop {
        if argc == counter { break; }
        arg = argv[counter].as_str();
        match arg {
            "-h" | "--help" => {
                config.has_task = false;
                print_usage(&program_name);
                break;
            },
            "-V" | "--version" => {
                config.has_task = false;
                println!("{program_name} {VERSION}");
                break;
            },
            "-d" | "--download" => config.download = true,
            "-q" | "--quality" => {
                if argc > counter+1 {
                    counter += 1;
                    let quality = argv[counter].as_str();
                    match quality {
                        "360" | "480" | "720" | "1080" => config.quality = quality.parse()?,
                        _ => {
                            config.has_task = false;
                            error_msg("Invalid quality.");
                            print_usage(&program_name);
                        }
                    }
                }
            },
            "-r" | "--range" => {
                if argc > counter+1 {
                    counter += 1;
                    let range: Vec<_> = argv[counter].split('-').collect();
                    if range.len() == 2 {
                        config.range_start = range[0].parse()?;
                        config.range_end = range[1].parse()?;
                    } else {
                        error_msg("Invalid format for range.");
                        print_usage(&program_name);
                    }
                } else {
                    config.has_task = false;
                    error_msg("Invalid format for range.");
                    print_usage(&program_name);
                }
            },
            x => {
                error_msg(&format!("Unknown option \"{x}\".")); 
                print_usage(&program_name);
            }
        }
        counter += 1;
    }
    Ok(config)
}

fn error_msg(msg: &str) {
    eprintln!("{COLOR_RED_BOLD}{}{COLOR_RESET}\n", msg);
}

fn print_usage(program_name: &str) {
    let usage = format!(r#"Description:
  CLI tool to stream and download Anime

Usage: 
  {program_name} [OPTIONS]

Options:
  -h, --help           Print help
  -V, --version        Print version
  -d, --download       Download anime
  -q, --quality        Specify quality
                       Available qualities are 360, 480, 720 & 1080
  -r, --range          Specify episode range separated by dash(-), ranges are inclusive"#);
    println!("{}", usage);
}

pub fn user_input(prompt: &str) -> Result<String, Box<dyn Error>> {
    stdout().write(prompt.as_bytes())?;
    stdout().flush()?;
    let mut anime_name = String::new();
    stdin().read_line(&mut anime_name)?;
    Ok(anime_name.trim().to_string())
}

pub fn sanitize_filename(filename: &str) -> String {
    let illegal_chars = [' ', '/', '\\', '?', '%', '*', ':', '|', '"', '<', '>', '.'];

    let sanitized = filename.chars()
        .map(|c| if illegal_chars.contains(&c) { '_' } else { c })
        .collect::<String>();

    // Remove control characters (ASCII 0-31) and Unicode replacement character (U+FFFD)
    let sanitized = sanitized.chars()
        .filter(|c| !c.is_control())
        .collect::<String>();

    // Trim any leading or trailing whitespace and periods
    let sanitized = sanitized.trim_start_matches('.').trim_end_matches('.').trim();

    // Ensure the filename is not empty after trimming
    if sanitized.is_empty() {
        return String::from("unnamed_file");
    }

    sanitized.to_lowercase()
}

pub fn decode_provider_id(resp: &str) -> String {
    let chars = resp.chars().collect::<Vec<char>>();
    let mut output = String::new();
    let mut counter = 0;
    let length = chars.len();
    let (mut x, mut y, mut r, mut n, mut a, mut ox, mut oy);
    while counter < length-1 {
        ox = chars[counter];
        oy = chars[counter+1]; 
        x = hexchar_to_i32(ox); 
        y = hexchar_to_i32(oy); 
        r = (8-y).rem_euclid(16);
        n = 16*x + (r+15).rem_euclid(16) + 1; 
        a = (64-n).rem_euclid(128);
        output.push(char:: from (a as u8)); 
        counter += 2;
    }
    output.replace("clock", "clock.json") 
}

fn hexchar_to_i32(ch: char) -> i32 {
    match ch {
        '0'..='9' => (ch as u8 - b'0') as i32,
        'a'..='f' => (ch as u8 - b'a' + 10) as i32,
        'A'..='F' => (ch as u8 - b'A' + 10) as i32,
        _ => panic!("Invalid hexadecimal character"),
    }
}
