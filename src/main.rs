use std::cmp;
use std::env;
use std::fs;
use std::io;

use std::collections::HashSet;
use std::fmt::Write;

const FMT_DISPLAY_WIDTH: usize = 6;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
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

fn wc_fmt(counts: &Vec<usize>) -> String {
    let mut fmt = String::new();
    let width = if counts.len() > 1 {
        FMT_DISPLAY_WIDTH
    } else {
        0
    };
    for count in counts {
        write!(fmt, "{:>width$} ", count).expect("Failed to write bytes");
    }
    fmt
}

fn add_option(opt: WcCliOpt, opts: &mut Vec<WcCliOpt>, seen: &mut HashSet<WcCliOpt>) {
    if !seen.contains(&opt) {
        seen.insert(opt.clone());
        opts.push(opt);
    }
}

fn wc(contents: &String, opts: &Vec<WcCliOpt>) -> Vec<usize> {
    let mut counts: Vec<usize> = Vec::new();

    for opt in opts.iter() {
        match opt {
            WcCliOpt::CountBytes => counts.push(contents.bytes().len()),
            WcCliOpt::CountCharacters => counts.push(contents.chars().count()),
            WcCliOpt::CountLines => counts.push(contents.lines().count()),
            WcCliOpt::MaxLineLength => {
                counts.push(contents.lines().map(|line| line.len()).max().unwrap_or(0))
            }
            WcCliOpt::CountWords => counts.push(
                contents
                    .lines()
                    .map(|line| line.split_whitespace().count())
                    .sum(),
            ),
        }
    }
    counts
}

fn main() {
    // Files
    let mut files: Vec<String> = Vec::new();

    // CLI Options
    let mut read_stdin = false;
    let mut opts: Vec<WcCliOpt> = Vec::new();

    // Parsing
    let mut seen: HashSet<WcCliOpt> = HashSet::new();
    for arg in env::args().skip(1).into_iter() {
        match arg.as_str() {
            "-c" | "--bytes" => add_option(WcCliOpt::CountBytes, &mut opts, &mut seen),
            "-m" | "--chars" => add_option(WcCliOpt::CountCharacters, &mut opts, &mut seen),
            "-l" | "--lines" => add_option(WcCliOpt::CountLines, &mut opts, &mut seen),
            "-L" | "--max-line-length" => add_option(WcCliOpt::MaxLineLength, &mut opts, &mut seen),
            "-w" | "--words" => add_option(WcCliOpt::CountWords, &mut opts, &mut seen),
            "-" => read_stdin = true,
            "--version" => return version(),
            "--help" => return help(),
            s if s.starts_with("--files0-from=") => add_input_files(s, &mut files),
            s if s.starts_with("-") => return invalid_opt(s),
            file => files.push(file.to_string()),
        }
    }

    // Default options
    if opts.is_empty() {
        opts = vec![
            WcCliOpt::CountLines,
            WcCliOpt::CountWords,
            WcCliOpt::CountBytes,
        ];
    }

    // Implementation
    if read_stdin || files.is_empty() {
        let file = if read_stdin { "-" } else { "" };
        let contents = io::read_to_string(io::stdin()).expect("Unable to read from stdin");
        println!(
            "{}{:>FMT_DISPLAY_WIDTH$}",
            wc_fmt(&wc(&contents, &opts)),
            file
        );
    }

    let mut total: Vec<usize> = Vec::new();
    for file in files.iter() {
        if let Ok(metadata) = fs::metadata(file) {
            let contents = if metadata.is_file() {
                fs::read_to_string(file).expect(&format!("Unable to read {}", file))
            } else {
                println!("ccwc: {}: Is a directory", file);
                String::new()
            };
            let counts = wc(&contents, &opts);
            println!("{}{:>FMT_DISPLAY_WIDTH$}", wc_fmt(&counts), file);

            // Update total count
            if total.is_empty() {
                total = counts;
            } else {
                total = total.iter().zip(&counts).map(|(&t, &c)| t + c).collect();
            }
        } else {
            println!("ccwc: {}: No such file or directory", file);
        }
    }

    // Display total count
    if files.len() > 1 {
        println!("{}total", wc_fmt(&total));
    }
}
