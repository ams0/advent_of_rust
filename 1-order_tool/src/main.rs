use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let matches = Command::new("Order Tool")
        .version("1.0")
        .about("Sorts pairs of numbers in a file and calculates distances")
        .subcommand(
            Command::new("order")
                .about("Orders the pairs of numbers")
                .arg(
                    Arg::new("input")
                        .short('f')
                        .long("file")
                        .value_name("INPUT")
                        .help("Input file containing pairs of numbers")
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("Output file to save the ordered pairs")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("distance")
                .about("Calculates total distance between paired numbers")
                .arg(
                    Arg::new("input")
                        .short('f')
                        .long("file")
                        .value_name("INPUT")
                        .help("Input file containing pairs of numbers")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("order") {
        let input_file = matches.get_one::<String>("input").unwrap();
        let output_file = matches.get_one::<String>("output").unwrap();

        if let Err(e) = process_files(input_file, output_file) {
            eprintln!("Error: {}", e);
        }
    } else if let Some(matches) = matches.subcommand_matches("distance") {
        let input_file = matches.get_one::<String>("input").unwrap();

        match calculate_total_distance(input_file) {
            Ok(total_distance) => println!("Total distance: {}", total_distance),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn process_files(input_file: &str, output_file: &str) -> io::Result<()> {
    let input_path = Path::new(input_file);
    let output_path = Path::new(output_file);

    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);

    let mut pairs: Vec<(u32, u32)> = Vec::new();

    for line in buffered.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(a), Ok(b)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                pairs.push((a, b));
            }
        }
    }

    pairs.sort();

    let mut output = File::create(output_path)?;
    for (a, b) in pairs {
        writeln!(output, "{} {}", a, b)?;
    }

    Ok(())
}

fn calculate_total_distance(input_file: &str) -> io::Result<i32> {
    let input_path = Path::new(input_file);
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in buffered.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                left_list.push(left);
                right_list.push(right);
            }
        }
    }

    left_list.sort();
    right_list.sort();

    let total_distance: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    Ok(total_distance)
}