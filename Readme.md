# File Searcher with KMP Algorithm and Progress Bar

This utility is designed to search for specific keywords within files in a given directory using the Knuth-Morris-Pratt (KMP) string matching algorithm. It also provides a progress bar to visualize the directory scanning process.

## Features

- **File Reading with Time Measurement**: Reads the entire content of a file and measures the time taken.
- **KMP Algorithm Implementation**: Efficiently searches for patterns (keywords) in file contents using the KMP algorithm.
- **Directory Listing with Progress Bar**: Lists all files in a specified directory with a progress bar to indicate scanning progress.
- **CSV Keyword Loading**: Loads search keywords from a CSV file.

## Prerequisites

- Rust programming language installed.
- `indicatif` crate for progress bar functionality.

## Installation

Clone the repository and navigate into the project directory:

```bash
git clone <repository-url>
cd <project-directory>
