use regex::Regex;
use clap::Parser;

// #[derive(clap::ValueEnum)]
#[derive(clap::ValueEnum, Clone, PartialEq)]
enum Language {
    Python,
    R,
    Rust,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(value_enum)]
    // Name of the language the program is written in 
    #[arg(short, long)]
    language: Language, 
    
    // Number of times to run it
    // #[arg(short, long, default_value_t = 1)]
    // count: u8, 

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

    if args.language == Language::Python {
        println!("python");
        let comment_line = Regex::new(r"^#* ").unwrap();
        let trailing_comment = Regex::new(r"#").unwrap();
        let block_comment = Regex::new(r"'''").unwrap();
        check_lines(comment_line, trailing_comment, block_comment, lines);
    } else if args.language == Language::R {
        println!("r");
        let comment_line = Regex::new(r"^#* ").unwrap();
        let trailing_comment = Regex::new(r"#").unwrap();
        let block_comment = Regex::new(r"^#* ").unwrap();
        check_lines(comment_line, trailing_comment, block_comment, lines);
    } else if args.language == Language::Rust {
        println!("rust");
        let comment_line = Regex::new(r"^/* ").unwrap();
        let trailing_comment = Regex::new(r"//").unwrap();
        let block_start = Regex::new(r"/\*").unwrap();
        let _block_end = Regex::new(r"\*/").unwrap();
        check_lines(comment_line, trailing_comment, block_start, lines);
    } else {
        ()
    }

}

fn check_lines(comment_line: Regex, trailing_comment: Regex, block_comment: Regex, lines: Vec<&str>) {
    for line in &lines {
        if comment_line.is_match(&line) {
            println!("leading: {}", line);
        } else if trailing_comment.is_match(&line) {
            println!("trailing: {}", line);
        } else if block_comment.is_match(&line) {
            println!("block: {}", line);
        } else {
            println!("false: {}", line);
        }
    }
}
