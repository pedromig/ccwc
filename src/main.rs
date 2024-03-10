use std::cmp;
use std::env;
use std::fs;
use std::io;

use std::collections::HashSet;
use std::fmt::Write;

// TODO: Implement "total" count display when the user supplies multiple input files

const FMT_DISPLAY_WIDTH: usize = 6;

#[derive(Debug, Eq, Hash, PartialEq)]
enum WcCliOpt {
    CountBytes,
    CountCharacters,
    CountLines,
    CountWords,
    MaxLineLength,
}

fn version() {
    println!(
        r#"ccwc 0.1.0
Copyright (C) 2024 <pedromiguelrodrigues2000@gmail.com>
License MIT: The MIT License <https://opensource.org/license/mit>
This is free software: you are free to change and redistribute it.
The software is provided “as is”, without warranty of any kind.

Written by Pedro Rodrigues"#
    );
}

fn help() {
    println!(
        r#"ccwc - print newline, word, and byte counts for each file

Usage: ccwc [OPTIONS]... [FILE]...
   or: ccwc [OPTIONS]... --file0-from=F

Description:

Print newline, word and byte counts for each FILE, and total line
if more than one FILE is specified. A word is a non-zero-length sequence
of characters delimited by white space.

With no FILE or when FILE is - read standard input

The options below may be used to select which counts are printed, always 
in the following order: newline, word, character, byte, maximum line length.

Options: 
    -c, --bytes             Print the byte counts
    -m, --chars             Print the character counts
    -l, --lines             Print the newline counts
        --files0-from=F     Read input from the files specified by
                              NUL-terminated names in file F;
                              If F is - then read names from standard input
    -L, --max-line-length   Print the maximum display width
    -w, --words             Print the word counts
        --help              Display this help and exit 
        --version           Output version information and exit"#
    )
}

fn invalid_opt(opt: &str) {
    let (_, opt) = opt.split_at("-".len());
    println!(
        "ccwc: invalid option -- '{}'",
        &opt[..cmp::min(opt.len(), 1)]
    );
    println!("Try 'ccwc --help' for more information");
}

fn add_input_files(opt: &str, files: &mut Vec<String>) {
    let (_, input) = opt.split_at("--files0-from=".len());
    let contents = if input == "-" {
        io::read_to_string(io::stdin()).expect("Failed to read from stdin")
    } else {
        fs::read_to_string(input).expect(&format!("Failed to read from {}", input))
    };
    files.extend(contents.split('\0').map(String::from));
}

fn wc(contents: &String, opts: &Vec<WcCliOpt>) -> (String, (usize, usize, usize, usize, usize)) {
    let mut fmt = String::new();
    let mut counts = (0, 0, 0, 0, 0);

    for opt in opts.iter() {
        match opt {
            WcCliOpt::CountBytes => {
                counts.0 = contents.bytes().len();
                write!(fmt, "{:>FMT_DISPLAY_WIDTH$}", counts.0)
                    .expect("Failed to write the number of bytes");
            }
            WcCliOpt::CountCharacters => {
                counts.1 = contents.chars().count();
                write!(fmt, "{:>FMT_DISPLAY_WIDTH$}", counts.1)
                    .expect("Failed to write the number of characters");
            }
            WcCliOpt::CountLines => {
                counts.2 = contents.lines().count();
                write!(fmt, "{:>FMT_DISPLAY_WIDTH$}", counts.2)
                    .expect("Failed to write the number of lines");
            }
            WcCliOpt::MaxLineLength => {
                let mut mx = 0;
                for line in contents.lines() {
                    mx = cmp::max(line.len(), mx);
                }
                counts.3 = mx;
                write!(fmt, "{:>FMT_DISPLAY_WIDTH$}", mx)
                    .expect("Failed to write the max line length");
            }
            WcCliOpt::CountWords => {
                let mut words = 0;
                for line in contents.lines() {
                    words += line.split_whitespace().count();
                }
                counts.4 = words;
                write!(fmt, "{:>FMT_DISPLAY_WIDTH$}", words)
                    .expect("Failed to write the number of words");
            }
        }
        fmt.push_str(" ");
    }
    (fmt, counts)
}

fn main() {
    // Files
    let mut files: Vec<String> = Vec::new();

    // CLI Options
    let mut read_stdin = false;
    let mut opts: HashSet<WcCliOpt> = HashSet::new();

    // Parsing
    for arg in env::args().skip(1).into_iter() {
        match arg.as_str() {
            "-c" | "--bytes" => {
                opts.insert(WcCliOpt::CountBytes);
            }
            "-m" | "--chars" => {
                opts.insert(WcCliOpt::CountCharacters);
            }
            "-l" | "--lines" => {
                opts.insert(WcCliOpt::CountLines);
            }
            "-L" | "--max-line-length" => {
                opts.insert(WcCliOpt::MaxLineLength);
            }
            "-w" | "--words" => {
                opts.insert(WcCliOpt::CountWords);
            }
            "-" => {
                read_stdin = true;
            }
            "--version" => {
                return version();
            }
            "--help" => {
                return help();
            }
            s if s.starts_with("--files0-from=") => add_input_files(s, &mut files),
            s if s.starts_with("-") => {
                return invalid_opt(s);
            }
            file => files.push(file.to_string()),
        }
    }

    // Default options
    let opts: Vec<WcCliOpt> = if opts.is_empty() {
        vec![
            WcCliOpt::CountLines,
            WcCliOpt::CountWords,
            WcCliOpt::CountBytes,
        ]
    } else {
        opts.into_iter().collect()
    };

    // Implementation
    if read_stdin || files.is_empty() {
        let file = if read_stdin { "-" } else { "" };
        let contents = io::read_to_string(io::stdin()).expect("Unable to read from stdin");
        let (fmt, _) = wc(&contents, &opts);
        println!("{}{:>FMT_DISPLAY_WIDTH$}", fmt, file);
    }

    for file in files.iter() {
        if let Ok(metadata) = fs::metadata(file) {
            let contents = if metadata.is_file() {
                fs::read_to_string(file).expect(&format!("Unable to read {}", file))
            } else {
                println!("ccwc: {}: Is a directory", file);
                String::new()
            };
            let (fmt, _) = wc(&contents, &opts);
            println!("{}{:>FMT_DISPLAY_WIDTH$}", fmt, file);
        } else {
            println!("ccwc: {}: No such file or directory", file);
        }
    }
}
