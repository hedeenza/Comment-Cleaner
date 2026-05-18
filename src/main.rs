use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

// Import necessary elements from lib.rs
use comment_cleaner::{Args, check_language};

fn main() {
    // Start the program benchmarking timer
    let program_start = Instant::now();

    // Parse the command line arguments using Clap
    let args = Args::parse();

    // Assign the input file
    let input_file = File::open(&args.file).unwrap();
    let input_reader = BufReader::new(input_file);
    // Create the output file
    let output_file = File::create("cleaned_script").expect("Could not create output file");

    // Run the main program logic
    check_language(args, input_reader, output_file);

    // Stop the program benchmarking timer
    let elapsed = program_start.elapsed();
    // Print the time
    println!("Comments cleaned in {:.2?}", elapsed);
}
