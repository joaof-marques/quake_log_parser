# Quake 3 Log Parser

This project is a Quake 3 Log Parser built using Rust.
## Running

To run this project, follow these steps:

 1. Clone this repository:

```bash
  git clone git@github.com:joaof-marques/quake_log_parser.git
```
2. Change into the project directory

```bash
  cd parser_project
```

3. Run the project using:

```bash
  cargo run
```
## Features

This parser reads a file in the `parser_project` folder and extracts data from it. After processing, it prints the data for all matches, including:
- Game ID
- Total kill count for the match
- List of players
- Kill ranking
- Kill methods ranking

## Technology Stack

**Back-end:** Rust
## Output Examples

After execution, the parser will print the processed data for each match, showing detailed information such as the game ID, total kill count, list of players, kill ranking, and kill methods ranking.