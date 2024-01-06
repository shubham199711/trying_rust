use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    typescript: bool,

    #[arg(short, long)]
    directory: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let current_project = if args.typescript {
        "express-api-starter-ts"
    } else {
        "express-api-starter"
    };
    let repo_url: &str = &("https://github.com/w3cj/".to_owned() + &current_project);

    let status = Command::new("git")
        .args(["clone", repo_url, "--depth", "1", "--branch", "main"])
        .status()
        .expect("failed to execute process");

    if !status.success() {
        println!("failed to clone");
    }

    let mut git_path = PathBuf::from(&args.directory);
    git_path.push(current_project);
    git_path.push(".git");
    match fs::remove_dir_all(git_path) {
        Ok(_) => println!("removed .git/"),
        Err(_) => println!("failed to remove .git/"),
    }

    let status = Command::new("npm")
        .args([
            "install",
            "--prefix",
            &(args.directory + &current_project.to_string()),
        ])
        .status()
        .expect("failed to execute process");

    println!("process finished with: {status}");

    if !status.success() {
        println!("failed to install packages");
    }

    println!("Done");
    Ok(())
}
