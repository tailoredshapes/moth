we need an `--agent-help` command that recursively lists all commands and subcommands with arguments and descriptions so that an agent can get an get an overview of everything in one shot instead of iterating `cmd --help` then `cmd subscmd --help` etc

----- AI agent updates -------

Create an `--agent-help` command-line option on the root `moth` command. This option will recursively display all commands, subcommands, arguments, and options, with their descriptions, in a clean, indented, hierarchical format optimized for LLM agents.

## Specification

### Command Signature

```bash
moth --agent-help
```

- When this option is specified, the CLI displays a complete tree structure of all commands, subcommands, arguments, and options, then exits successfully.
- It displays names, descriptions, and whether arguments/options are required.

### Output Format

The output will structure the command tree recursively:

```
moth
  A simple file-based issue tracker

  Options:
    --agent-help                       Display recursive overview of all commands and subcommands
    --list-ids                         List all story IDs (for completion) (hidden)
    --list-statuses                    List all status names (for completion) (hidden)

  Subcommands:
    moth init                                 Initialize .moth/ directory
    moth new  [--severity ] [--no-edit] [--start] [--stdin]
      Create a new issue
    ...
```

### Design Decisions

- We add `--agent-help` as a root-level boolean option on the `Cli` struct using `clap`.
- The option is handled in `main.rs` before checking for other commands. If active, we recursively build and print the command tree directly from the `Cli::command()` metadata.
- Traversal is done recursively. Commands, subcommands, arguments, and options are indented clearly to represent hierarchy.
- A custom formatter is implemented to walk the `clap::Command` tree, since `clap`'s standard help renderer is designed for single-level commands.

### Implementation Details

- Define the `agent_help` flag in `src/main.rs`'s `Cli` structure.
- Add `src/cmd/agent_help.rs` (and declare it in `src/cmd/mod.rs`), implementing a `run` function that takes the `clap::Command` representing `moth`.
- The `run` function recursively formats each command, its arguments (arguments are split into positionals vs options/flags), and then recursively processes its subcommands with increased indentation.
- Write tests in `tests/features/agent_help.feature` (BDD feature) and `tests/e2e_shell_test.rs` to verify correct output and successful exit.

## Decisions

- Decided to implement recursive help as a new root flag `--agent-help` rather than a subcommand, to keep invocation simple.
- Chose to generate output in indented hierarchical format for LLM readability rather than raw clap help output.
- Rejected alternative of using existing `--help` output parsing due to complexity and fragility.
- Decided to place implementation abstract in the specification file, not in code comments.
- Ensured that tests include both BDD feature and shell test for verification.
