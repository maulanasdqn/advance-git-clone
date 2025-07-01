use clap::Parser;
use std::env;
use std::path::Path;
use std::process::{Command, exit};

#[derive(Parser)]
#[command(name = "adc")]
#[command(about = "Advanced Git Clone - Clone repositories with specific SSH keys")]
#[command(version = "0.1.0")]
struct Cli {
    #[arg(long, help = "SSH key name to use for cloning")]
    ssh: String,
    
    #[arg(help = "SSH URL of the repository to clone")]
    url: String,
    
    #[arg(help = "Directory name for the cloned repository (optional)")]
    directory: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    
    let ssh_key_path = format!("{}/.ssh/{}", 
        env::var("HOME").unwrap_or_else(|_| {
            eprintln!("Error: Could not determine HOME directory");
            exit(1);
        }),
        cli.ssh
    );
    
    if !Path::new(&ssh_key_path).exists() {
        eprintln!("Error: SSH key '{}' not found at path: {}", cli.ssh, ssh_key_path);
        exit(1);
    }
    
    let ssh_command = format!("ssh -i {} -o StrictHostKeyChecking=no", ssh_key_path);
    
    println!("Cloning repository with SSH key: {}", cli.ssh);
    println!("Repository URL: {}", cli.url);
    
    let mut cmd = Command::new("git");
    cmd.arg("clone")
       .arg(&cli.url)
       .env("GIT_SSH_COMMAND", &ssh_command);
    
    if let Some(dir) = &cli.directory {
        cmd.arg(dir);
    }
    
    match cmd.status() {
        Ok(status) => {
            if status.success() {
                println!("Repository cloned successfully!");
            } else {
                eprintln!("Error: Git clone failed with exit code: {:?}", status.code());
                exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to execute git command: {}", e);
            exit(1);
        }
    }
}
