use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "moth")]
#[command(about = "A simple file-based issue tracker", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// List all story IDs (for completion)
    #[arg(long, hide = true)]
    pub list_ids: bool,

    /// List all status names (for completion)
    #[arg(long, hide = true)]
    pub list_statuses: bool,

    /// Display recursive overview of all commands and subcommands
    #[arg(long)]
    pub agent_help: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    #[command(about = "Initialize .moth/ directory")]
    Init,

    #[command(about = "Create a new issue")]
    New {
        #[arg(help = "Issue title")]
        title: String,

        #[arg(short, long, help = "Severity (crit, high, med, low)")]
        severity: Option<String>,

        #[arg(long, help = "Skip opening editor")]
        no_edit: bool,

        #[arg(long, help = "Start the issue immediately (move to 'doing' status)")]
        start: bool,

        #[arg(long, help = "Read story body from stdin")]
        stdin: bool,
    },

    #[command(about = "List issues")]
    Ls {
        #[arg(short = 't', long, help = "Filter by status")]
        status: Option<String>,

        #[arg(short, long, help = "Show all including done")]
        all: bool,

        #[arg(short = 's', long, help = "Filter by severity (crit, high, med, low)")]
        severity: Option<String>,
    },

    #[command(about = "Show issue details")]
    Show {
        #[arg(help = "Issue ID (full or partial)")]
        id: Option<String>,
    },

    #[command(about = "Move issue to 'doing' status")]
    Start {
        #[arg(help = "Issue ID (full or partial)")]
        id: String,
    },

    #[command(about = "Move issue to 'done' status")]
    Done {
        #[arg(help = "Issue ID (full or partial)")]
        id: Option<String>,
    },

    #[command(about = "Move issue to specific status")]
    Mv {
        #[arg(help = "Issue ID (full or partial)")]
        id: String,

        #[arg(help = "Target status")]
        status: String,
    },

    #[command(about = "Edit issue in configured editor")]
    Edit {
        #[arg(help = "Issue ID (full or partial)")]
        id: String,
    },

    #[command(about = "Delete an issue")]
    Rm {
        #[arg(help = "Issue ID (full or partial)")]
        id: String,
    },

    #[command(about = "Extract story change history from git commits as CSV")]
    Report {
        #[arg(long, help = "Start from this commit (optional)")]
        since: Option<String>,

        #[arg(long, help = "End at this commit (optional)")]
        until: Option<String>,
    },

    #[command(about = "Set priority order for a story")]
    Priority {
        #[arg(help = "Issue ID (full or partial)")]
        id: String,

        #[arg(help = "Position: top, bottom, above, below, or number")]
        position: String,

        #[arg(help = "Other issue ID (required for above/below)")]
        other_id: Option<String>,

        #[arg(long, help = "Compact after repositioning")]
        compact: bool,

        #[arg(long, help = "Don't compact after repositioning")]
        no_compact: bool,
    },

    #[command(about = "Compact priority numbering in status")]
    Compact {
        #[arg(help = "Status to compact (optional, defaults to all prioritized)")]
        status: Option<String>,
    },

    #[command(about = "Change issue severity")]
    Severity {
        #[arg(help = "Issue ID (full or partial)")]
        id: String,

        #[arg(help = "Severity level (crit, high, med, low)")]
        level: String,
    },

    #[command(about = "Manage git commit hooks")]
    Hook {
        #[command(subcommand)]
        command: HookCommands,
    },

    #[command(about = "Generate shell completions")]
    Completions {
        #[arg(help = "Shell type: bash, zsh, or fish")]
        shell: String,
    },

    #[command(about = "Check for issue ID prefix in a message (used by git hook)")]
    Prefix {
        #[arg(help = "The message to check")]
        message: String,
    },

    #[command(about = "Update issue description from stdin")]
    Update {
        #[arg(help = "Issue ID (full or partial, defaults to current)")]
        id: Option<String>,
    },

    #[command(about = "Create CLAUDE.md with moth agent guide for LLM assistants")]
    Claude {
        #[arg(long, help = "Overwrite existing CLAUDE.md")]
        force: bool,

        #[arg(long, help = "Append to existing CLAUDE.md")]
        append: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum HookCommands {
    #[command(about = "Install prepare-commit-msg hook")]
    Install {
        #[arg(long, help = "Overwrite existing hook")]
        force: bool,

        #[arg(long, help = "Append to existing hook")]
        append: bool,
    },

    #[command(about = "Uninstall prepare-commit-msg hook")]
    Uninstall,
}
