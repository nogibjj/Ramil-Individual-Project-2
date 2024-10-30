[![CI/CD](https://github.com/nogibjj/Ramil-Individual-Project-2/actions/workflows/main.yaml/badge.svg)](https://github.com/nogibjj/Ramil-Individual-Project-2/actions/workflows/main.yaml)

# Project #2: Rust CLI Binary with SQLite

## Overview
This project is a Command Line Interface (CLI) tool developed in Rust that interacts with a SQLite database. It allows users to perform CRUD (Create, Read, Update, Delete) operations efficiently, loads the CSV to SQLlite database, demonstrating Rust’s capabilities in system programming. The project is optimized to generate a binary using Github Actions, ensuring a fast and lightweight executable.

## Key Features
- **Rust CLI Application**: Written entirely in Rust, showcasing the language’s syntax and unique features.
- **SQLite Database**: Includes a SQLite database connection for CRUD operations.
- **Optimized Binary**: The Rust code is compiled into an optimized binary, made available for download as a GitHub Actions artifact.
- **Automated CI/CD Pipeline**: GitHub Actions handle testing, building, and linting the code.

## Performance Measurements
This project showcases fundamental CRUD operations, but not suited for direct performance benchmarking. You can find the Python code implementing these operations in this repository ([GithubLink](https://github.com/nogibjj/Ramil-Python-Script-interacting-with-SQL-Database)), where it also runs quickly. Although no precise performance metrics are included, Rust is significantly faster than Python under heavy workloads. Rust’s speed advantage comes from its compilation into a binary format that runs directly on the OS, utilizing multiple cores and threads. Python, on the other hand, relies on an interpreter that processes code during runtime which takes additional resources and time, and its Global Interpreter Lock (GIL) limits true multicore execution. Sample performance measurements for Rust are provided in the images section below.

## How LLM was Utilized
Throughout the development process, I used a Language Learning Model (LLM) for guidance on Rust's syntax and best practices, particularly for SQLite integration and structuring efficient database interactions. The LLM was instrumental in code review and optimizing error handling in Rust.

## Dependencies
- **Rust** (>=1.60): 
- **SQLite**: Pre-installed on most systems; 
- **RustLibraries**: reqwest, rusqlite, csv, clap, rustc-serialize, sys-info

## Setup and Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/nogibjj/Ramil-Individual-Project-2.git
   cd Ramil-Individual-Project-2
   ```

2. Build the Rust binary:
   ```bash
   cargo build --release
   ```

3. Run the optimized binary:
   ```bash
   ./target/release/rust_ind8
   ```
4. Format the Rust code:
   ```bash
    make format
   ```
5. Lint the Rust code:
   ```bash
    make lint
   ```

6. Check the Rust code:
   ```bash
    make check
   ```

7. Check the Rust code:
   ```bash
    make check
   ```

## Running the Program

The CLI tool allows for basic CRUD operations within the SQLite database. Use the following commands to interact with the database:

```bash
cargo run -- insert --player "LeBron James" --position "SF" --id "LBJ23" --draft-year 2003 --projected-spm 10.5 --superstar 0.95 --starter 0.85 --role-player 0.75 --bust 0.05
```

# Read entries
```bash
cargo run -- read --id "LBJ23"
```

# Update an entry
```bash
cargo run -- update --id "LBJ23" --new-player "LeBron James" --new-position "PF" --new-draft-year 2004 --new-projected-spm 11.0
```

# Delete an entry
```bash
cargo run -- delete --id "LBJ23"
```


## GitHub Actions CI/CD
This project includes a GitLab CI/CD pipeline configured with the following:
- **Testing**: Runs unit tests to validate code functionality.
- **Building**: Compiles the Rust code and generates an optimized binary.
- **Linting**: Ensures code quality and Rust best practices.
- The optimized binary is available as an artifact after each successful pipeline run.

## Youtube Video 
For a quick demonstration and walkthrough of the CLI tool, watch this video: [YouTube video](https://youtu.be/JWNB-1dY3C0).

## Images

**Rust CLI Insert**

![Run Example Console Output](https://github.com/nogibjj/Ramil-Individual-Project-2/blob/1dd2e0365ddb516342a0e5ffb4a45a15142e5b90/images/Insert_metrics.png)


**Rust CLI Read**

![Run Example Console Output](https://github.com/nogibjj/Ramil-Individual-Project-2/blob/1dd2e0365ddb516342a0e5ffb4a45a15142e5b90/images/Read_metrics.png)


**Rust CLI Update**

![Run Example Console Output](https://github.com/nogibjj/Ramil-Individual-Project-2/blob/1dd2e0365ddb516342a0e5ffb4a45a15142e5b90/images/Update_metrics.png)


**Rust CLI Delete**

![Run Example Console Output](https://github.com/nogibjj/Ramil-Individual-Project-2/blob/1dd2e0365ddb516342a0e5ffb4a45a15142e5b90/images/Delete_metrics.png)


**Rust CLI Read After Delete**

![Run Example Console Output](https://github.com/nogibjj/Ramil-Individual-Project-2/blob/1dd2e0365ddb516342a0e5ffb4a45a15142e5b90/images/read%20after%20delete.png)

