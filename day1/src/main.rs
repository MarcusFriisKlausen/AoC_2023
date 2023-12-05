use std::env;

mod star1;
mod star2;

fn main() {
    if let Some(_arg1) = env::args().nth(2) {
        println!("Main only takes 1 argument!");
        return ();
    } 

    if let Some(_arg1) = env::args().nth(1) {
        let sum_of_calibrations1: u32 = star1::decode_file(&_arg1);
        let sum_of_calibrations2: u32 = star2::decode_file_str(&_arg1);
        println!("Sum of calibrations excl. string literals = {}", sum_of_calibrations1);
        println!("Sum of calibrations incl. string literals = {}", sum_of_calibrations2);
    }
    return ();
}
