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
    ];

    check_language(args, lines);

}


fn check_language(args: Args, lines: Vec<&str>) {
    if args.language == Language::Python {
        println!("python");
        let full_comment = Regex::new(r"^#* ").unwrap();
        let trailing_comment = Regex::new(r"#").unwrap();
        let block_comment = Regex::new(r"'''").unwrap();
        check_included(args, full_comment, trailing_comment, block_comment, lines);
    } else if args.language == Language::R {
        println!("r");
        let full_comment = Regex::new(r"^#* ").unwrap();
        let trailing_comment = Regex::new(r"#").unwrap();
        let block_comment = Regex::new(r"^#* ").unwrap();
        check_included(args, full_comment, trailing_comment, block_comment, lines);
    } else if args.language == Language::Rust {
        println!("rust");
        let full_comment = Regex::new(r"^/* ").unwrap();
        let trailing_comment = Regex::new(r"//").unwrap();
        let block_comment = Regex::new(r"/\*").unwrap();
        //let _block_end = Regex::new(r"\*/").unwrap();
        check_included(args, full_comment, trailing_comment, block_comment, lines);
    } else {
        ()
    }
}

fn check_included(args: Args, full_comment: regex::Regex, trailing_comment: regex::Regex, block_comment: regex::Regex, lines: Vec<&str>) {
    println!("INCLUDES: {:?}", args.include);
    if args.include.contains(&CommentTypes::FullLine) && args.include.contains(&CommentTypes::TrailingLine) && args.include.contains(&CommentTypes::Block) {
        println!("Full Line + Trailing Line + Block");
        let filters = vec![full_comment.clone(), trailing_comment.clone(), block_comment.clone()];
        filter_lines(filters, lines);
    } else if args.include.contains(&CommentTypes::FullLine) && args.include.contains(&CommentTypes::TrailingLine) {
        println!("Full Line + Trailing Line");
        let filters = vec![full_comment.clone(), trailing_comment.clone()];
        filter_lines(filters, lines);
    } else if args.include.contains(&CommentTypes::FullLine) && args.include.contains(&CommentTypes::Block) {
        println!("Full Line + Block");
        let filters = vec![full_comment.clone(), block_comment.clone()];
        filter_lines(filters, lines);
    } else if args.include.contains(&CommentTypes::TrailingLine) && args.include.contains(&CommentTypes::Block) {
        println!("Trailing Line + Block");
        let filters = vec![trailing_comment.clone(), block_comment.clone()];
        filter_lines(filters, lines);
    } else if args.include.contains(&CommentTypes::FullLine) {
        println!("FullLine");
        let filters = vec![full_comment.clone()];
        filter_lines(filters, lines);
    } else if args.include.contains(&CommentTypes::TrailingLine) {
        println!("TrailingLine");
        let filters = vec![trailing_comment.clone()];
        filter_lines(filters, lines);
    } else if args.include.contains(&CommentTypes::Block) {
        println!("Block");
        let filters = vec![block_comment.clone()];
        filter_lines(filters, lines);
    } else {
        println!("[ Error ]");
    }
}

fn filter_lines(filters: Vec<Regex>, lines: Vec<&str>) {
    for line in &lines {
        for filter in &filters {
            if filter.is_match(&line) {
                println!("{}: {}", filter, line);
            }
        }
    }
}
