use std::fs::File;
use std::error::Error;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}

fn words_to_digits<'a>() -> HashMap<&'a str, &'a str> {
    HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ])
}

fn find_all_indexes(line: &str, string: &str) -> Vec<usize> {
    let mut indexes: Vec<usize> = Vec::new();
    let mut index_tracker = 0;

    while let Some(index) = line[index_tracker..].find(string) {
        let global_index = index_tracker + index;
        indexes.push(global_index);
        index_tracker = global_index + 1;
    }

    indexes
}

fn day1(file_path: String, words_to_digits: &HashMap<&str, &str>) -> Result<(), Box<dyn Error>> {
    if let Ok(lines) = read_lines(file_path) {
        let mut sum: u32 = 0;

        for line in lines.flatten() {
            let mut indexes_of_words = HashMap::new();

            for (word, digit) in words_to_digits.iter() {
                let words_indexes = find_all_indexes(&line, word);
                let digits_indexes = find_all_indexes(&line, digit);
                
                for word_index in words_indexes {
                    indexes_of_words.insert(word_index, word);
                }

                for digit_index in digits_indexes {
                    indexes_of_words.insert(digit_index, digit);
                }
            }

            let indexes = indexes_of_words
                .keys()
                .cloned()
                .collect::<Vec<usize>>();

            if indexes.len() > 0 {
                let &min_index = indexes.iter().min().unwrap();
                let &max_index = indexes.iter().max().unwrap();
                let &&value_min_index = indexes_of_words.get(&min_index).unwrap();
                let &&value_max_index = indexes_of_words.get(&max_index).unwrap();
                let mut number = String::new();

                if let Some(&value) = words_to_digits.get(value_min_index) {
                    number.push_str(value);
                } else {
                    number.push_str(value_min_index)
                }

                if let Some(&value) = words_to_digits.get(value_max_index) {
                    number.push_str(value);
                } else {
                    number.push_str(value_max_index);
                }

                println!("{}", number);

                sum += number.parse::<u32>().unwrap();
            }
        }

        println!("{}", sum);
    }

    Ok(())
}

pub fn run(file_path: String) -> Result<(), Box<dyn Error>> {
    let words_to_digits = words_to_digits();
    day1(file_path, &words_to_digits)?;

    Ok(())
}
