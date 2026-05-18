use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(clap::ValueEnum, Clone, PartialEq)]
pub enum Language {
    Python,
    R,
    Rust,
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
pub enum CommentTypes {
    Full,
    Trailing,
    Block,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File to process
    #[arg(short, long)]
    pub file: String,

    #[clap(value_enum)]
    /// Name of the language the program is written in
    #[arg(short, long)]
    language: Language,

    /// Comment type to remove
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    include: Vec<CommentTypes>,
}

pub fn check_language<R>(args: Args, input_reader: BufReader<R>, output_file: File)
where
    R: std::io::Read,
{
    if args.language == Language::Python {
        let comment_character = "#";
        let full_comment = Regex::new(r"^#* ").unwrap();
        let trailing_comment = Regex::new(r"^[^#].*#").unwrap();
        let block_comment = Regex::new(r"'''").unwrap();
        check_included(
            args,
            comment_character,
            full_comment,
            trailing_comment,
            block_comment,
            input_reader,
            output_file,
        );
    } else if args.language == Language::R {
        let comment_character = "#";
        let full_comment = Regex::new(r"^#* ").unwrap();
        let trailing_comment = Regex::new(r"^[^#].*#").unwrap();
        let block_comment = Regex::new(r"^#* ").unwrap();
        check_included(
            args,
            comment_character,
            full_comment,
            trailing_comment,
            block_comment,
            input_reader,
            output_file,
        );
    } else if args.language == Language::Rust {
        let comment_character = "//";
        let full_comment = Regex::new(r"^/* ").unwrap();
        let trailing_comment = Regex::new(r"^[^//].*//").unwrap();
        let block_comment = Regex::new(r"/\*|\*/").unwrap();
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
    let mut keep_line = true;

    if args.include.contains(&CommentTypes::Full)
        && args.include.contains(&CommentTypes::Trailing)
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
    } else if args.include.contains(&CommentTypes::Full) {
        for line in input_reader.lines() {
            let line = line.unwrap();
            if full_comment.is_match(&line.trim()) {
                continue;
            } else {
                let _ = writeln!(output_file, "{}", &line);
            }
        }
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
