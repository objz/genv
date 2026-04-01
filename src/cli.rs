use clap::{Parser, Subcommand, ValueEnum};
use color_eyre::eyre::Result;
use std::path::PathBuf;

use crate::completions;
use crate::env::{
    default_env_file, load_env_map, quote_fish, quote_posix, save_env_map, validate_key, EnvMap,
};
use crate::logging as log;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Manage global environment variables (portable, no systemd)"
)]
struct Cli {
    /// Override the managed env file (default: ~/.config/envm/env)
    #[arg(long)]
    file: Option<PathBuf>,

    #[command(subcommand)]
    cmd: CommandKind,
}

#[derive(Subcommand, Debug)]
enum CommandKind {
    /// Add a new variable
    Add { key: String, value: String },

    /// Edit an existing variable
    Edit { key: String, value: String },

    /// Remove a variable
    Remove { key: String },

    /// List variables managed by this file
    List,

    /// Print exports for the current shell. Use with: eval "$(envm export)"
    Export {
        /// Force a shell style
        #[arg(long, value_enum)]
        shell: Option<ShellKind>,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: CompletionShell,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum ShellKind {
    Posix,
    Fish,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum CompletionShell {
    Bash,
    Zsh,
    Fish,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let env_file = default_env_file(cli.file.as_deref())?;

    let mut map: EnvMap = match load_env_map(&env_file) {
        Ok(m) => m,
        Err(err)
            if err.to_string().contains("No such file")
                || err.to_string().contains("not found")
                || err.to_string().contains("open") =>
        {
            EnvMap::default()
        }
        Err(e) => return Err(e),
    };

    match &cli.cmd {
        CommandKind::Add { key, value } => {
            if let Err(e) = validate_key(key) {
                log::error(e.to_string());
                return Ok(());
            }
            if let Some(existing) = map.get(key) {
                if existing == value {
                    log::warn(format!("variable {} already set to the same value", key));
                    return Ok(());
                }
                print!(
                    "variable {} exists with value '{}'. Overwrite? [y/N]: ",
                    key, existing
                );
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
                    log::warn(format!("aborted adding variable {}", key));
                    return Ok(());
                }
            }
            map.insert(key.clone(), value.clone());
            save_env_map(&env_file, &map)?;
            log::success(format!("added {}={}", key, value));
        }

        CommandKind::Edit { key, value } => {
            if let Err(e) = validate_key(key) {
                log::error(e.to_string());
                return Ok(());
            }
            let Some(existing) = map.get(key) else {
                log::error(format!("key {} does not exist", key));
                return Ok(());
            };
            if existing == value {
                log::warn(format!("variable {} already has that value", key));
                return Ok(());
            }
            map.insert(key.clone(), value.clone());
            save_env_map(&env_file, &map)?;
            log::success(format!("edited {}={}", key, value));
        }

        CommandKind::Remove { key } => {
            if let Err(e) = validate_key(key) {
                log::error(e.to_string());
                return Ok(());
            }
            if map.remove(key).is_some() {
                save_env_map(&env_file, &map)?;
                log::success(format!("removed {}", key));
            } else {
                log::warn(format!("key {} is not present", key));
            }
        }

        CommandKind::List => {
            let mut items: Vec<(String, String)> =
                map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            items.sort_by(|a, b| a.0.cmp(&b.0));
            if items.is_empty() {
                println!("# {} (no variables)", env_file.display());
            } else {
                println!("# {}", env_file.display());
                let width = items.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
                for (k, v) in items {
                    println!("{:width$} = {}", k, v, width = width);
                }
            }
        }

        CommandKind::Export { shell } => {
            let flavor = shell.unwrap_or_else(detect_shell);
            let mut keys: Vec<_> = map.keys().cloned().collect();
            keys.sort();
            match flavor {
                ShellKind::Posix => {
                    for k in keys {
                        let v = map.get(&k).unwrap();
                        println!("export {}={}", k, quote_posix(v));
                    }
                }
                ShellKind::Fish => {
                    for k in keys {
                        let v = map.get(&k).unwrap();
                        println!("set -gx {} {}", k, quote_fish(v));
                    }
                }
            }
        }

        CommandKind::Completions { shell } => {
            let mut stdout = io::stdout();
            match shell {
                CompletionShell::Bash => completions::generate_bash(&mut stdout)?,
                CompletionShell::Zsh => completions::generate_zsh(&mut stdout)?,
                CompletionShell::Fish => completions::generate_fish(&mut stdout)?,
            }
        }
    }

    Ok(())
}

fn detect_shell() -> ShellKind {
    if std::env::var_os("FISH_VERSION").is_some() {
        return ShellKind::Fish;
    }
    if let Some(shell) = std::env::var_os("SHELL")
        && let Some(s) = shell.to_str()
        && s.ends_with("/fish")
    {
        return ShellKind::Fish;
    }
    ShellKind::Posix
}
