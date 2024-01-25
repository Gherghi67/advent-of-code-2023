
use std::error::Error;

use crate::utils::read_lines;

struct Rgb {
    blue: u32,
    green: u32,
    red: u32,
}

impl Rgb {
    pub fn initialise() -> Rgb {
        Rgb {
            blue: 0,
            green: 0,
            red: 0,
        }
    }

    pub fn increment_by_color(&mut self, color: &str, value: u32) {
        match color {
            "blue" => { self.blue += value; },
            "green" => { self.green += value; },
            "red" => { self.red += value; },
            _ => {},
        }
    }
}

fn is_game_valid(second_half_splitted: &Vec<&str>) -> bool {
    for part in second_half_splitted {
        let mut rgb_sum = Rgb::initialise();
        let sub_parts: Vec<&str> = part.split(", ").collect();

        for sub_part in sub_parts {
            let sub_sub_part: Vec<&str> = sub_part.split(" ").collect();
            let value = sub_sub_part[0].parse::<u32>().unwrap();
            rgb_sum.increment_by_color(sub_sub_part[1], value);

            if rgb_sum.blue > 14 || rgb_sum.green > 13 || rgb_sum.red > 12 {
                return false;
            }
        };
    }

    true
}

fn max_rgb_sum(second_half_splitted: Vec<&str>) -> Rgb {
    let mut max_rgb_sum = Rgb::initialise();

    for part in second_half_splitted {
        let mut rgb_sum = Rgb::initialise();
        let sub_parts: Vec<&str> = part.split(", ").collect();

        for sub_part in sub_parts {
            let sub_sub_part: Vec<&str> = sub_part.split(" ").collect();
            let value = sub_sub_part[0].parse::<u32>().unwrap();
            rgb_sum.increment_by_color(sub_sub_part[1], value);
        }

        if rgb_sum.blue > max_rgb_sum.blue {
            max_rgb_sum.blue = rgb_sum.blue;
        }

        if rgb_sum.green > max_rgb_sum.green {
            max_rgb_sum.green = rgb_sum.green;
        }

        if rgb_sum.red > max_rgb_sum.red {
            max_rgb_sum.red = rgb_sum.red;
        }
    }

    max_rgb_sum
}

fn day2(file_path: String) -> Result<(), Box<dyn Error>> {
    if let Ok(lines) = read_lines(file_path) {
        let mut sum_of_ids: u32 = 0;
        let mut sum_of_set_powers: u32 = 0;

        for line in lines.flatten() {
            let main_part_splitted: Vec<&str>  = line.split(": ").collect();
            let first_half_splitted = main_part_splitted[0].split(" ");
            let game_id = first_half_splitted.collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
            let second_half_splitted: Vec<&str> = main_part_splitted[1].split("; ").collect();
            let is_game_valid = is_game_valid(&second_half_splitted);

            if is_game_valid {
                sum_of_ids += game_id;
            }

            let max_rgb_sum = max_rgb_sum(second_half_splitted);
            sum_of_set_powers += max_rgb_sum.blue * max_rgb_sum.green * max_rgb_sum.red;
        }

        println!("Sum of ids: {}", sum_of_ids);
        println!("Sum of set of powers: {}", sum_of_set_powers);
    }

    Ok(())
}

pub fn run(file_path: String) -> Result<(), Box<dyn Error>> {
    let _ = day2(file_path);

    Ok(())
}