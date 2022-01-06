use chrono::{DateTime, NaiveDateTime, Utc};
use rand::Rng;
use std::process::Command;
use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num_commits = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    let start_date = NaiveDateTime::parse_from_str("2019-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")?.and_utc();
    let end_date = Utc::now();

    for i in 0..num_commits {
        let random_time = rand_time(start_date, end_date);
        let commit_message = format!("Generated commit {}: {}", i + 1, random_time.to_rfc3339());

        let file_path = Path::new("data.txt");
        fs::write(file_path, format!("Commit time: {}\n", commit_message))?;

        run_git_command(&["add", "."])?;
        run_git_command(&["commit", "-m", &commit_message, "--date", &random_time.to_rfc3339()])?;

        println!("Create commit: {}", commit_message);
    }

    run_git_command(&["push"])?;

    Ok(())
}

fn rand_time(min: DateTime<Utc>, max: DateTime<Utc>) -> DateTime<Utc> {
    let mut rng = rand::thread_rng();
    let min_ts = min.timestamp();
    let max_ts = max.timestamp();
    let random_ts = rng.gen_range(min_ts..=max_ts);
    DateTime::<Utc>::from_timestamp(random_ts, 0).unwrap()
}

fn run_git_command(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("git").args(args).output()?;
    if !output.status.success() {
        return Err(format!("Git command failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    Ok(())
}