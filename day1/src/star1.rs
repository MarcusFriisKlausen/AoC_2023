use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn is_digit(n: &char) -> bool {
    // possible digits
    let digits: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    if digits.contains(n) {
        return true;
    }
    return false;
}

pub fn reverse(s: &String) -> String {
    return s.chars().rev().collect();
}

fn line_decoder(line: &String) -> u32 {
    let mut num_str: String = String::new();
    let mut ch_unbrwd: char;
    let mut ch: &char;


    for i in 0..line.len() {
        ch_unbrwd = line.chars().nth(i).unwrap();
        ch = &ch_unbrwd;
        if is_digit(ch) {
            num_str.push(*ch);
            break;
        }
    }
    
    let enil: &String = &reverse(line);
    
    for i in 0..enil.len() {
        ch_unbrwd = enil.chars().nth(i).unwrap();
        ch = &ch_unbrwd;
        if is_digit(ch) {
            num_str.push(*ch);
            break;
        }
    }

    if num_str.len() != 2 {
        println!("ERROR: Number isn't 2 digits!");
        return 0;
    }

    let num: u32 = num_str.parse::<u32>().unwrap(); 
    return num;
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    return buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
}

pub fn decode_file(filename: impl AsRef<Path>) -> u32 {
    let _lines = lines_from_file(filename);
    let mut sum_of_lines:u32 = 0;

    for line in _lines {
        let line_brwd = &line;
        sum_of_lines += line_decoder(line_brwd);
    }
    return sum_of_lines;
}
