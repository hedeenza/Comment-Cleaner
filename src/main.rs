use clap::Parser;
use regex::Regex;

#[derive(clap::ValueEnum, Clone, PartialEq)]
enum Language {
    Python,
    R,
    Rust,
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
enum CommentTypes {
    FullLine,
    TrailingLine,
    Block,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(value_enum)]
    /// Name of the language the program is written in 
    #[arg(short, long)]
    language: Language, 

    /// Comment type to remove
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    include: Vec<CommentTypes>,

}

fn main() {
    let args = Args::parse();

    let lines = vec![
        "This has no comment",
        "/ One line to start",
        "// True Comment",
        "/// This has three",
        "//// This has four!",
        "This one has division/and such",
        "This one has division / and such",
        "This has real code // Then a comment later",
        "This has real code //Then a comment later",
        "This has real code ///Then a comment later",
        "This has no comment",
        "# One line to start",
        "## True Comment",
        "### This has three",
        "#### This has four!",
        "This one has division#and such",
        "This one has division # and such",
        "This has real code ## Then a comment later",
        "This has real code ##Then a comment later",
        "This has real code ###Then a comment later",
        "/* this one starts a block comment",
        "''' this one starts a block comment",
        "In between a block comment 1",
        "In between a block comment 2",
        "In between a block comment 3",
        "In between a block comment 4",
        "In between a block comment 5",
        "''' this one ends a block comment",
        "*/ this one ends a block comment",
    ];

    check_language(args, lines);

}


fn check_language(args: Args, lines: Vec<&str>) {
    if args.language == Language::Python {
        println!("python");
        let full_comment = Regex::new(r"^#* ").unwrap();
        let trailing_comment = Regex::new(r"^[^#].*#").unwrap();
        let block_comment = Regex::new(r"'''").unwrap();
        check_included(args, full_comment, trailing_comment, block_comment, lines);
    } else if args.language == Language::R {
        println!("r");
        let full_comment = Regex::new(r"^#* ").unwrap();
        let trailing_comment = Regex::new(r"^[^#].*#").unwrap();
        let block_comment = Regex::new(r"^#* ").unwrap();
        check_included(args, full_comment, trailing_comment, block_comment, lines);
    } else if args.language == Language::Rust {
        println!("rust");
        let full_comment = Regex::new(r"^/* ").unwrap();
        let trailing_comment = Regex::new(r"^[^//].*//").unwrap();
        let block_comment = Regex::new(r"/\*|\*/").unwrap();
        check_included(args, full_comment, trailing_comment, block_comment, lines);
    } else {
        ()
    }
}

fn check_included(args: Args, full_comment: regex::Regex, trailing_comment: regex::Regex, block_comment: regex::Regex, lines: Vec<&str>) {
    let mut keep_line = true;

    if args.include.contains(&CommentTypes::FullLine) && args.include.contains(&CommentTypes::TrailingLine) && args.include.contains(&CommentTypes::Block) {
        for line in lines {
            if keep_line == true {
                if block_comment.is_match(&line) {
                    keep_line = !keep_line;
                    continue
                } else if full_comment.is_match(&line) {
                    continue
                } else if trailing_comment.is_match(&line) {
                    continue
                } else {
                    println!("LINE: {}", line);
                }
            } else if keep_line == false {
                if block_comment.is_match(&line) {
                    keep_line = !keep_line;
                    continue
                }
            } else {
                println!("It is unclear whether to keep or skip the line");
            }
        }

    } else if args.include.contains(&CommentTypes::FullLine) && args.include.contains(&CommentTypes::TrailingLine) {
        for line in lines {
            if full_comment.is_match(&line) {
                continue
            } else if trailing_comment.is_match(&line) {
                continue
            } else {
                println!("LINE: {}", line);
            }
        }

    } else if args.include.contains(&CommentTypes::FullLine) && args.include.contains(&CommentTypes::Block) {
        for line in lines {
            if keep_line == true {
                if block_comment.is_match(&line) {
                    keep_line = !keep_line;
                    continue
                } else if full_comment.is_match(&line) {
                    continue
                } else {
                    println!("LINE: {}", line);
                }
            } else if keep_line == false {
                if block_comment.is_match(&line) {
                    keep_line = !keep_line;
                    continue
                }
            } else {
                println!("It is unclear whether to keep or skip the line");
            }
        }

    } else if args.include.contains(&CommentTypes::TrailingLine) && args.include.contains(&CommentTypes::Block) {
        for line in lines {
            if keep_line == true {
                if block_comment.is_match(&line) {
                    keep_line = !keep_line;
                    continue
                } else if trailing_comment.is_match(&line) {
                    continue
                } else {
                    println!("LINE: {}", line);
                }
            } else if keep_line == false {
                if block_comment.is_match(&line) {
                    keep_line = !keep_line;
                    continue
                }
            } else {
                println!("It is unclear whether to keep or skip the line");
            }
        }

    } else if args.include.contains(&CommentTypes::FullLine) {
        for line in lines {
            if full_comment.is_match(&line) {
                continue
            } else {
                println!("LINE: {}", line);
            }
        }

    } else if args.include.contains(&CommentTypes::TrailingLine) {
        for line in lines {
            if trailing_comment.is_match(&line) {
                continue
            } else {
                println!("LINE: {}", line);
            }
        }

    } else if args.include.contains(&CommentTypes::Block) {
        for line in lines {
            if keep_line == true {
                if block_comment.is_match(&line) {
                    keep_line = !keep_line;
                    continue
                } else {
                    println!("LINE: {}", line);
                }
            } else if keep_line == false {
                if block_comment.is_match(&line) {
                    keep_line = !keep_line;
                    continue
                }
            } else {
                println!("It is unclear whether to keep or skip the line");
            }
        }

    } else {
        println!("No Comment Types Provided");
    }
}
