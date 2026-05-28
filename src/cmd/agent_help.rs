use anyhow::Result;
use clap::{ArgAction, Command};

/// Prints a concise hierarchical view of the `moth` CLI.
///
/// The root command name (`moth`) and its description are printed first.
/// Root level options are listed under an `Options:` heading.
/// Subcommands are listed under a `Subcommands:` heading. Each subcommand line
/// shows the full command prefixed with `moth` and any optional flags formatted
/// as `[--flag]`. The description of the subcommand (its `about` text) is printed
/// on the following line.
pub fn run(cmd: &mut Command) -> Result<()> {
    // Root command name and about text
    println!("{}", cmd.get_name());
    if let Some(about) = cmd.get_about() {
        println!("  {}", about);
    }

    // Root level options (flags that take a value are shown with a placeholder)
    print_root_options(cmd);

    // Subcommands with the required formatting
    print_subcommands(cmd);
    Ok(())
}

fn print_root_options(cmd: &Command) {
    let mut opts = Vec::new();
    for arg in cmd.get_arguments() {
        // Skip positional arguments only
        if arg.get_short().is_none() && arg.get_long().is_none() {
            continue; // positional, ignored for root options list
        }
        let mut parts = Vec::new();
        if let Some(short) = arg.get_short() {
            parts.push(format!("-{}", short));
        }
        if let Some(long) = arg.get_long() {
            parts.push(format!("--{}", long));
        }
        let placeholder = match arg.get_action() {
            ArgAction::Set | ArgAction::Append => {
                let name = arg
                    .get_value_names()
                    .and_then(|v| v.first())
                    .map(|s| s.to_uppercase())
                    .unwrap_or_else(|| arg.get_id().as_str().to_uppercase());
                format!(" <{}>", name)
            }
            _ => String::new(),
        };
        opts.push(format!("{}{}", parts.join(", "), placeholder));
    }
    if !opts.is_empty() {
        println!("  Options:");
        for opt in opts {
            println!("    {}", opt);
        }
    }
}

fn print_subcommands(cmd: &Command) {
    // Print the top‑level subcommands header
    println!("  Subcommands:");
    // Start recursive printing from root with depth 1 (one level of indentation)
    recurse_subcommands(cmd, cmd.get_name(), 1);
}

// Recursive helper to print subcommands at any depth
fn recurse_subcommands(cmd: &Command, prefix: &str, depth: usize) {
    let mut subs: Vec<&Command> = cmd.get_subcommands().collect();
    subs.sort_by_key(|c| c.get_name());
    if subs.is_empty() {
        return;
    }
    let indent = "  ".repeat(depth);
    for sub in subs {
        // Print description as a comment if available
        if let Some(about) = sub.get_about() {
            println!("{}# {}", indent, about.to_string().trim());
        }
        // Collect optional flag representations
        let mut opt_parts = Vec::new();
        for arg in sub.get_arguments() {
            if arg.get_short().is_none() && arg.get_long().is_none() {
                continue;
            }
            if let Some(long) = arg.get_long() {
                opt_parts.push(format!("[--{}]", long));
            } else if let Some(short) = arg.get_short() {
                opt_parts.push(format!("[--{}]", short));
            }
        }
        // Build full command line
        let cmd_line = format!(
            "{}{} {}{}",
            indent,
            prefix,
            sub.get_name(),
            if opt_parts.is_empty() {
                "".to_string()
            } else {
                format!(" {}", opt_parts.join(" "))
            }
        );
        println!("{}", cmd_line);
        println!();
        // Recurse into deeper subcommands
        let new_prefix = format!("{} {}", prefix, sub.get_name());
        recurse_subcommands(sub, &new_prefix, depth + 1);
    }
}
