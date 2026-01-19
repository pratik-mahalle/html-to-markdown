#![allow(clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]

mod args;
mod convert;
mod output;
mod utils;
mod validators;

use args::{Cli, Shell};
use clap::Parser;
use convert::{build_conversion_options, perform_conversion};
use output::{output_debug_info, write_output};
use std::fs;
use std::io::{self, Read, Write as IoWrite};
use std::path::PathBuf;
use utils::{DEFAULT_USER_AGENT, decode_bytes, fetch_url};

fn generate_completions(shell: Shell) {
    use clap::CommandFactory;
    use clap_complete::{Shell as ClapShell, generate};

    let mut cmd = Cli::command();
    let shell = match shell {
        Shell::Bash => ClapShell::Bash,
        Shell::Zsh => ClapShell::Zsh,
        Shell::Fish => ClapShell::Fish,
        Shell::PowerShell => ClapShell::PowerShell,
        Shell::Elvish => ClapShell::Elvish,
    };

    generate(shell, &mut cmd, "html-to-markdown", &mut io::stdout());
}

fn generate_man_page() -> Result<(), String> {
    use clap::CommandFactory;

    let cmd = Cli::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer = Vec::new();
    man.render(&mut buffer)
        .map_err(|e| format!("Failed to generate man page: {e}"))?;

    io::stdout()
        .write_all(&buffer)
        .map_err(|e| format!("Failed to write man page: {e}"))?;

    Ok(())
}

fn read_input(cli: &Cli) -> Result<String, Box<dyn std::error::Error>> {
    let html = match cli.input.as_deref() {
        _ if cli.url.is_some() => {
            let user_agent = cli.user_agent.as_deref().unwrap_or(DEFAULT_USER_AGENT);
            let fetched = fetch_url(cli.url.as_deref().unwrap(), user_agent, &cli.encoding)?;
            output_debug_info(cli, &format!("Fetched {} bytes from URL", fetched.len()));
            fetched
        }
        None | Some("-") => {
            let mut buffer = Vec::new();
            io::stdin()
                .read_to_end(&mut buffer)
                .map_err(|e| format!("Error reading from stdin: {e}"))?;
            let decoded = decode_bytes(&buffer, &cli.encoding)?;
            output_debug_info(cli, &format!("Read {} bytes from stdin", decoded.len()));
            decoded
        }
        Some(path) => {
            let path = PathBuf::from(path);
            let bytes = fs::read(&path).map_err(|e| format!("Error reading file '{}': {}", path.display(), e))?;
            let decoded = decode_bytes(&bytes, &cli.encoding)?;
            output_debug_info(
                cli,
                &format!("Read {} bytes from file '{}'", decoded.len(), path.display()),
            );
            decoded
        }
    };
    Ok(html)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(shell) = cli.generate_completion {
        generate_completions(shell);
        return Ok(());
    }

    if cli.generate_man {
        generate_man_page()?;
        return Ok(());
    }

    let html = read_input(&cli)?;
    let options = build_conversion_options(&cli);
    let output_content = perform_conversion(&html, options, &cli)?;
    write_output(cli.output.clone(), &output_content)?;

    Ok(())
}
