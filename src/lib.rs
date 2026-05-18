use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

// Create an enum to hold the language options
#[derive(clap::ValueEnum, Clone, PartialEq)]
pub enum Language {
    Python,
    R,
    Rust,
}

// Create an enum to hold the comment type option
#[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
pub enum CommentTypes {
    Full,
    Trailing,
    Block,
}

// Create a struct to hold the command line arguments
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    // Create a field for the input file
    /// File to process
    #[arg(short, long)]
    pub file: String,

    // Create a field for the language of the input script
    #[clap(value_enum)]
    /// Name of the language the program is written in
    #[arg(short, long)]
    language: Language,

    // Create a field for the types of comments the user wants to remove
    /// Comment type to remove
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    include: Vec<CommentTypes>,
}

// Function to check which language was selected
pub fn check_language<R>(args: Args, input_reader: BufReader<R>, output_file: File)
where
    R: std::io::Read,
{
    // If Python was selected...
    if args.language == Language::Python {
        // Set the comment character parameters for Python
        let comment_character = "#";
        let full_comment = Regex::new(r"^#* ").unwrap();
        let trailing_comment = Regex::new(r"^[^#].*#").unwrap();
        let block_comment = Regex::new(r"'''").unwrap();
        // Run the funciton to check which comment types are to be processed
        check_included(
            args,
            comment_character,
            full_comment,
            trailing_comment,
            block_comment,
            input_reader,
            output_file,
        );
    // If R was selected...
    } else if args.language == Language::R {
        // Set the comment character parameters for R
        let comment_character = "#";
        let full_comment = Regex::new(r"^#* ").unwrap();
        let trailing_comment = Regex::new(r"^[^#].*#").unwrap();
        let block_comment = Regex::new(r"^#* ").unwrap();
        // Run the funciton to check which comment types are to be processed
        check_included(
            args,
            comment_character,
            full_comment,
            trailing_comment,
            block_comment,
            input_reader,
            output_file,
        );
    // If Rust was selected...
    } else if args.language == Language::Rust {
        // Set the comment character parameters for Rust
        let comment_character = "//";
        let full_comment = Regex::new(r"^/* ").unwrap();
        let trailing_comment = Regex::new(r"^[^//].*//").unwrap();
        let block_comment = Regex::new(r"/\*|\*/").unwrap();
        // Run the funciton to check which comment types are to be processed
        check_included(
            args,
            comment_character,
            full_comment,
            trailing_comment,
            block_comment,
            input_reader,
            output_file,
        );
    }
}

// Function to check which comment types were selected
// And run the processing logic
pub fn check_included<R>(
    args: Args,
    comment_character: &str,
    full_comment: regex::Regex,
    trailing_comment: regex::Regex,
    block_comment: regex::Regex,
    input_reader: BufReader<R>,
    mut output_file: File,
) where
    R: std::io::Read,
{
    // Set the initial value state of keeping the input line for the output to true
    let mut keep_line = true;

    // If all 3 comment types were selected...
    if args.include.contains(&CommentTypes::Full)
        && args.include.contains(&CommentTypes::Trailing)
        && args.include.contains(&CommentTypes::Block)
    {
        // For each line in the input file...
        for line in input_reader.lines() {
            // Unwrap the line from the reader
            let line = line.unwrap();
            // If we want to keep the line...
            if keep_line {
                // If the block comment start/end is a match...
                if block_comment.is_match(&line.trim()) {
                    // Flip the keep_line state
                    // This is the "detect the beginning of a block comment" functionality
                    // It doesn't matter what's between the beginning and end of a block comment in
                    // the event we want block comments removed
                    keep_line = !keep_line;
                    // Skip the line
                    continue;
                // If the full line comment is a match...
                } else if full_comment.is_match(&line.trim()) {
                    // Skip the line
                    continue;
                // If the trailing line comment is a match...
                } else if trailing_comment.is_match(&line.trim()) {
                    // If there is an index where the comment character is located...
                    if let Some(index) = line.find(comment_character) {
                        // Keep the line only until the character
                        let clean_line = &line[0..index];
                        // Write the line, free of the comment, to the output
                        let _ = writeln!(output_file, "{}", &clean_line);
                    }
                } else {
                    // In a line without any comments, write the line to the output as is
                    let _ = writeln!(output_file, "{}", &line);
                }
            // If we don't want to keep the line...
            } else if !keep_line {
                // If the block comment start/end is a match...
                if block_comment.is_match(&line.trim()) {
                    // Flip the keep_line state
                    // This is the "detect the end of a block comment" functionality
                    keep_line = !keep_line;
                    // Skip the line
                    continue;
                }
            } else {
                // In an ambiguos state, print an error message
                println!("It is unclear whether to keep or skip the line");
            }
        }
    // If full line and trailing line comments were selected...
    } else if args.include.contains(&CommentTypes::Full)
        && args.include.contains(&CommentTypes::Trailing)
    {
        for line in input_reader.lines() {
            let line = line.unwrap();
            if full_comment.is_match(&line.trim()) {
                continue;
            } else if trailing_comment.is_match(&line.trim()) {
                if let Some(index) = line.find(comment_character) {
                    let clean_line = &line[0..index];
                    let _ = writeln!(output_file, "{}", &clean_line);
                }
            } else {
                let _ = writeln!(output_file, "{}", &line);
            }
        }
    // If full line and block comments were selected...
    } else if args.include.contains(&CommentTypes::Full)
        && args.include.contains(&CommentTypes::Block)
    {
        for line in input_reader.lines() {
            let line = line.unwrap();
            if keep_line {
                if block_comment.is_match(&line.trim()) {
                    keep_line = !keep_line;
                    continue;
                } else if full_comment.is_match(&line.trim()) {
                    continue;
                } else {
                    let _ = writeln!(output_file, "{}", &line);
                }
            } else if !keep_line {
                if block_comment.is_match(&line.trim()) {
                    keep_line = !keep_line;
                    continue;
                }
            } else {
                println!("It is unclear whether to keep or skip the line");
            }
        }
    // If trailing line and block comments were selected...
    } else if args.include.contains(&CommentTypes::Trailing)
        && args.include.contains(&CommentTypes::Block)
    {
        for line in input_reader.lines() {
            let line = line.unwrap();
            if keep_line {
                if block_comment.is_match(&line.trim()) {
                    keep_line = !keep_line;
                    continue;
                } else if trailing_comment.is_match(&line.trim()) {
                    if let Some(index) = line.find(comment_character) {
                        let clean_line = &line[0..index];
                        let _ = writeln!(output_file, "{}", &clean_line);
                    }
                } else {
                    let _ = writeln!(output_file, "{}", &line);
                }
            } else if !keep_line {
                if block_comment.is_match(&line.trim()) {
                    keep_line = !keep_line;
                    continue;
                }
            } else {
                println!("It is unclear whether to keep or skip the line");
            }
        }
    // If only full line comments were selected...
    } else if args.include.contains(&CommentTypes::Full) {
        for line in input_reader.lines() {
            let line = line.unwrap();
            if full_comment.is_match(&line.trim()) {
                continue;
            } else {
                let _ = writeln!(output_file, "{}", &line);
            }
        }
    // If only trailing line comments were selected...
    } else if args.include.contains(&CommentTypes::Trailing) {
        for line in input_reader.lines() {
            let line = line.unwrap();
            if trailing_comment.is_match(&line.trim()) {
                if let Some(index) = line.find(comment_character) {
                    let clean_line = &line[0..index];
                    let _ = writeln!(output_file, "{}", &clean_line);
                }
            } else {
                let _ = writeln!(output_file, "{}", &line);
            }
        }
    // If only block comments were selected...
    } else if args.include.contains(&CommentTypes::Block) {
        for line in input_reader.lines() {
            let line = line.unwrap();
            if keep_line {
                if block_comment.is_match(&line.trim()) {
                    keep_line = !keep_line;
                    continue;
                } else {
                    let _ = writeln!(output_file, "{}", &line);
                }
            } else if !keep_line {
                if block_comment.is_match(&line.trim()) {
                    keep_line = !keep_line;
                    continue;
                }
            } else {
                println!("It is unclear whether to keep or skip the line");
            }
        }
    } else {
        println!("No Comment Types Provided");
    }
}
