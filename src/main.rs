use clap::{ArgGroup, Parser};

/// Arguments to the CLI application
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(group(
    ArgGroup::new("proxies")
        .required(true)
        .multiple(true)
        .args(["gpg", "ssh"]),
    ))]
struct Cli {
    /// Launch the gpg-agent proxy
    #[arg(short, long)]
    gpg: bool,
    /// Launch the ssh-pageant proxy
    #[arg(short, long)]
    ssh: bool,
}

/// The WSL GPG/SSH Pageant CLI
fn main() {
    // Parse the CLI args
    let cli = Cli::parse();

    // Launch the gpg-agent proxy
    if cli.gpg {
        println!("Launching the gpg-agent proxy...");
    }

    // Launch the ssh-pageant proxy
    if cli.ssh {
        println!("Launching the ssh-pageant proxy...");
    }
}
