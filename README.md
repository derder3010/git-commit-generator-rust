# Git Commit Generator (Rust)

This is a simple command-line tool written in Rust that generates a specified number of Git commits with random timestamps within a given date range. This can be useful for simulating repository activity, testing Git tools, or creating visualizations of commit history.

## Features

*   Generates multiple commits with random timestamps.
*   Configurable number of commits via command-line arguments.
*   Uses the `chrono` crate for accurate date and time handling.
*   Robust error handling.
*   Clear output messages.

## Prerequisites

*   [Rust](https://www.rust-lang.org/tools/install) (including Cargo, Rust's package manager)
*   Git

## Installation

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/derder3010/git-commit-generator-rust.git
    cd git-commit-generator-rust
    ```

2.  **Build the project:**

    ```bash
    cargo build --release # Release build for better performance
    ```

## Usage

1.  **Initialize a Git repository (if you haven't already):**

    ```bash
    git init
    ```

2.  **(Optional) Create an initial commit:**

    ```bash
    touch README.md
    git add .
    git commit -m "Initial commit"
    ```

3.  **Run the generator:**

    *   To generate the default 10 commits:

        ```bash
        cargo run --release
        ```

    *   To generate a specific number of commits (e.g., 50):

        ```bash
        cargo run --release -- 50
        ```

    *   To run the built binary directly:

        ```bash
        ./target/release/git-commit-generator-rust 100 # Example: 100 commits
        ```

4.  **(Optional) Push the commits to a remote repository:**

    ```bash
    git remote add origin <your-remote-repository-url>
    git push origin main # Or git push origin master
    ```

## Configuration (Date Range)

The start date is currently hardcoded in `src/main.rs` to January 1, 2019. To change the start date, modify this line:

```rust
let start_date = NaiveDateTime::parse_from_str("2019-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")?.and_utc();
```
*    Change "2019-01-01 00:00:00" to your desired start date and time in the format YYYY-MM-DD HH:MM:SS. The end date is always the current time.

### Example

*   To generate 25 commits with random timestamps between January 1, 2019, and the current time, you would run:
```Bash

cargo run --release -- 25
```

### Ethical Considerations

*   Please use this tool responsibly. Artificially inflating your contribution graph can be misleading and is generally discouraged. The contribution graph is intended to reflect genuine work and activity.


