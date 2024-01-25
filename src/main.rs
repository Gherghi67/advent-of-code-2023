use std::env;
use std::process;

mod day1;
mod day2;
mod utils;


fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let semi_file_path = "src/inputs/day";
    let file_path = format!("{semi_file_path}{day}.txt");
    
    if let Err(e) = day2::run(file_path) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
