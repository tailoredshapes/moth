use clap::{CommandFactory, Parser};
use clap_complete::{Shell, generate};
use moth::cmd;
use std::env::args;
use std::io;
use std::process;

use moth::cli::{Cli, Commands, HookCommands};

fn main() {
    let cli = Cli::parse();

    // Handle hidden completion helper flags
    if cli.list_ids {
        list_story_ids();
        return;
    }

    if cli.list_statuses {
        list_statuses();
        return;
    }

    if cli.agent_help {
        let mut cmd = Cli::command();
        if let Err(e) = cmd::agent_help::run(&mut cmd) {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
        return;
    }

    let Some(command) = cli.command else {
        eprintln!("Error: No command specified. Use --help for usage information.");
        process::exit(1);
    };

    let command_name = args().nth(1).unwrap_or_default();

    if let Err(e) = cmd::lifecycle_hooks::run_hooks(&command_name, "before") {
        eprintln!("Error running before hook: {}", e);
        process::exit(1);
    }

    let result = match command {
        Commands::Init => cmd::init::run(),
        Commands::New {
            title,
            severity,
            no_edit,
            start,
            stdin,
        } => {
            let body = if stdin {
                use std::io::Read;
                let mut content = String::new();
                io::stdin()
                    .read_to_string(&mut content)
                    .expect("Failed to read from stdin");
                Some(content)
            } else {
                None
            };
            cmd::new::run(&title, severity.as_deref(), no_edit, start, body)
        }
        Commands::Ls {
            status,
            all,
            severity,
        } => {
            let sev_filter = severity
                .as_deref()
                .map(|s| s.parse())
                .transpose()
                .unwrap_or_else(|e| {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                });
            cmd::list::run(status.as_deref(), all, sev_filter)
        }
        Commands::Show { id } => cmd::show::run(id.as_deref()),
        Commands::Start { id } => cmd::start::run(&id),
        Commands::Done { id } => cmd::done::run(id.as_deref()),
        Commands::Mv { id, status } => cmd::mv::run(&id, &status),
        Commands::Edit { id } => cmd::edit::run(&id),
        Commands::Rm { id } => cmd::rm::run(&id),
        Commands::Report { since, until } => cmd::report::run(since.as_deref(), until.as_deref()),
        Commands::Priority {
            id,
            position,
            other_id,
            compact,
            no_compact,
        } => {
            let compact_opt = if compact {
                Some(true)
            } else if no_compact {
                Some(false)
            } else {
                None
            };
            cmd::priority::run(&id, &position, other_id.as_deref(), compact_opt)
        }
        Commands::Compact { status } => cmd::priority::compact(status.as_deref()),
        Commands::Severity { id, level } => {
            let sev = level.parse().unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                process::exit(1);
            });
            cmd::severity::run(&id, sev)
        }
        Commands::Hook { command } => match command {
            HookCommands::Install { force, append } => cmd::hook::install(force, append),
            HookCommands::Uninstall => cmd::hook::uninstall(),
        },
        Commands::Completions { shell } => {
            generate_completions(&shell);
            return;
        }
        Commands::Update { id } => {
            use std::io::Read;
            let mut content = String::new();
            io::stdin()
                .read_to_string(&mut content)
                .expect("Failed to read from stdin");
            cmd::update::run(id.as_deref(), content)
        }
        Commands::Prefix { message } => cmd::prefix::check(&message),
        Commands::Claude { force, append } => {
            if append {
                cmd::claude::append()
            } else {
                cmd::claude::run(force)
            }
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    if let Err(e) = cmd::lifecycle_hooks::run_hooks(&command_name, "after") {
        eprintln!("Error running after hook: {}", e);
        process::exit(1);
    }
}

fn list_story_ids() {
    use std::fs;

    // Fast path: scan filesystem without full validation
    let Ok(mut current) = std::env::current_dir() else {
        return;
    };

    // Find .moth directory
    let moth_dir = loop {
        let moth = current.join(".moth");
        if moth.is_dir() {
            break moth;
        }
        if !current.pop() {
            return; // No .moth found, silently exit
        }
    };

    // Scan all status directories
    let Ok(entries) = fs::read_dir(&moth_dir) else {
        return;
    };

    let mut ids = Vec::new();
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_dir() || path.file_name().unwrap().to_str().unwrap().starts_with('.') {
            continue;
        }

        // Scan files in this status directory
        let Ok(files) = fs::read_dir(&path) else {
            continue;
        };

        for file in files.filter_map(Result::ok) {
            let filename = file.file_name();
            let name = filename.to_string_lossy();
            if !name.ends_with(".md") {
                continue;
            }

            // Extract ID from filename: [NNN-]ID-priority-slug.md
            let parts: Vec<&str> = name.trim_end_matches(".md").split('-').collect();
            if parts.len() < 3 {
                continue;
            }

            // Check if first part is a number (priority order)
            let id_idx = if parts[0].parse::<u32>().is_ok() {
                1
            } else {
                0
            };
            if parts.len() > id_idx {
                ids.push(parts[id_idx].to_string());
            }
        }
    }

    // Print unique IDs
    ids.sort();
    ids.dedup();
    for id in ids {
        println!("{}", id);
    }
}

fn list_statuses() {
    use moth::config::Config;

    // Try to load config, fail silently if not available
    let Ok(config) = Config::load() else {
        return;
    };

    for status in &config.statuses {
        println!("{}", status.name);
    }
}

fn generate_completions(shell_name: &str) {
    let shell = match shell_name.to_lowercase().as_str() {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        _ => {
            eprintln!(
                "Error: Unknown shell '{}'. Supported: bash, zsh, fish",
                shell_name
            );
            process::exit(1);
        }
    };

    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "moth", &mut io::stdout());
}
