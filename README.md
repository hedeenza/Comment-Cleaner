# Source Code Comment Cleaner

## Use Case:
- I heavily comment my source code. I find it incredibly helpful for learning unfamiliar modules/packages/crates, features, and techniques. If anyone ends up using anything I contribute as a learning tool, it will be there for them. Others find heavy commenting to be cluttering and unnecessary. This tool is for them. Instead of removing the comments manually, should you want to, use this tool to produce a copy of the source code free of any commments.

## Get the Tool
- The pre-compiled binary (for Linux and Windows) and source code are available in "Releases".
- macOS users will need to compile from source.

## Running the CLI
`$ ./comment-cleaner --file <FILE-NAME> --language <LANGUAGE> --include <CommentType(s)>`

- Select file (-f / --file): Enter the name of the file you want to clean.
- Supported languages (-l / --language):  Python, R, Rust (<python>, <r>, <rust>)
- Comment types (-i / --include): full line comments, trailing comments, block comments (<full>, <trailing>, <block>)
- Output as "<cleaned_script>"
- Ensure the program has executable permissions.

## Building from Source
Navigate to the project root directory.
- If using cargo: `$ cargo build --release`
- If not using cargo: `$ rustc -0 src/main.rs`

The executable binary should then be available in `./target/release/`

## Running the CLI from anywhere in your file system
Add the following lines to your `.bashrc` file:
```
~/.bashrc
# Source Code Comment Cleaner
export PATH="$PATH:/home/path/to/directory/where/this/program/lives"

alias cc="comment_cleaner"
```

## License
This program is distributed under the terms of a GNU GPLv3 license. See LICENSE.md for details.
