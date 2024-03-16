# Build Your Own wc Tool

This project is a solution to the "Build Your Own wc Tool" challenge
implemented in Rust.

## Introduction

The Unix command line tool `wc` is a widely used utility that counts the number
of bytes, words, and lines in a file. This challenge aims to implement a
simplified version of `wc` following the Unix philosophies of simplicity and
modularity.

## Challenge Overview

The challenge consists of several steps, each requiring the implementation of a
specific feature of `wc`:

- **Step One**: Implement counting bytes (`-c` option).
- **Step Two**: Implement counting lines (`-l` option).
- **Step Three**: Implement counting words (`-w` option).
- **Step Four**: Implement counting characters (`-m` option).
- **Step Five**: Implement default counting (equivalent to `-c`, `-l`, and `-w`
options).
- **The Final Step**: Implement reading from standard input if no filename is
specified.

This challenge is available on
[CodingChallenges.fyi](https://codingchallenges.fyi/challenges/challenge-wc/).

## Usage

To use the `ccwc` utility, follow the steps below:

1. Clone the repository.
2. Navigate to the project directory.
3. Build the project using `cargo build`.
4. Run the executable with appropriate options and input file.

Example usage:

```bash 
# Count bytes in a file 
ccwc -c file.txt

# Count lines in a file 
ccwc -l file.txt

# Count words in a file 
ccwc -w file.txt

# Count characters in a file 
ccwc -m file.txt

# Default count (bytes, lines, and words) 
ccwc file.txt

# Read from standard input 
cat file.txt | ccwc 
