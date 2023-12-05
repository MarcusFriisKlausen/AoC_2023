use std::char;

use std::path::Path;

use crate::star1 as s1;

static DIGITS_ARR: [&str; 10] = [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
            ];

fn convert_str_to_digit(s : &str) -> char {
    let index = DIGITS_ARR.iter().position(|&r| r == s).unwrap();
    return char::from_digit(index as u32, 10).unwrap();
}

fn is_in_string(s: &str) -> (bool, char) {
    for digit in DIGITS_ARR {
        if s.contains(digit) {
            return (true, convert_str_to_digit(digit));
        }
    }
    return (false, '0');
}

fn line_decoder_str(line: &String) -> u32 {
    let mut num_str: String = String::new();
    let mut ch_unbrwd: char;
    let mut ch: &char;
    let mut str_slice: &str;
    let num: u32;

    for i in 0..line.chars().count() {
        ch_unbrwd = line.chars().nth(i).unwrap();
        ch = &ch_unbrwd;
        str_slice = &line[..i + 1];
        println!("ch: {}\nstr: {}", ch, str_slice);

        if s1::is_digit(ch) {
            // println!("Found 1");
            num_str.push(*ch);
            break;
        } else {
            if is_in_string(str_slice).0 {
                // println!("Found 1");
                num_str.push(is_in_string(str_slice).1);
                break;
            }
        }
        
        if num_str.chars().count() == 1 {
            break;
        }
    }
    
    for j in 0..(line.chars().count()) {
        ch_unbrwd = line.chars().nth((line.chars().count() - 1) - j).unwrap();
        ch = &ch_unbrwd;
        str_slice = &line[((line.chars().count() - 1) - j)..(line.chars().count())];
        println!("ch: {}\nstr: {}", ch, str_slice);

        if s1::is_digit(ch) {
            //println!("Found 2");
            num_str.push(*ch);
            break;
        } else {
            if is_in_string(str_slice).0 {
                //println!("Found 2");
                num_str.push(is_in_string(str_slice).1);
                break;
            }
        }

        if num_str.chars().count() == 2 {
            break;
        }
    }
    println!("{}", num_str);
    if num_str.chars().count() > 2 {
        println!("ERROR: Too many digits!");
        return 0;
    } else if num_str.chars().count() < 2 {
        println!("ERROR: Not enough digits!");
        return 0;
    }
    
    num = num_str.parse::<u32>().unwrap();
    return num;
}

pub fn decode_file_str(filename: impl AsRef<Path>) -> u32 {
    let _lines = s1::lines_from_file(filename);
    let mut sum_of_lines:u32 = 0;

    for line in _lines {
        let line_brwd = &line;
        sum_of_lines += line_decoder_str(line_brwd);
    }
    return sum_of_lines;
}