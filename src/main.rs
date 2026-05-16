use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

use comment_cleaner::{Args, check_language};

fn main() {
    let program_start = Instant::now();

    let args = Args::parse();

    let input_file = File::open(&args.file).unwrap();
    let input_reader = BufReader::new(input_file);
    let output_file = File::create("cleaned_script").expect("Could not create output file");

    check_language(args, input_reader, output_file);

    let elapsed = program_start.elapsed();
    println!("Comments cleaned in {:.2?}", elapsed);
}
