Create a new command `update` that takes text from standard input and saves it to the issue's description. This command is primarily intended to be used by AI agents for keeping track of issue's specification as it evolves.

Update tests and relevant documentation parts including examples in CLAUDE.md and README.md

## Specification

### Command signature

```
moth update [id]
```

- `id` — optional, partial or full issue ID; defaults to current issue (reads `.moth/.current`)
- Content is read from **stdin** and replaces the entire issue file content
- Prints `Updated {id}: {title}` on success
- Fails with "No current issue" if no ID given and no current issue is set
- Fails with "No issue found" if the given ID does not match any issue

### Design decisions

- The entire file content is replaced (no merging/appending). This keeps the implementation simple and matches the agent use case where the agent rewrites the spec in full.
- The CLI reads stdin before dispatching to `cmd::update::run`, which accepts a `String` body — consistent with how `cmd::new::run` handles its `body: Option<String>` parameter. This allows tests to pass content directly without needing to mock stdin.
- The command works across all statuses (ready, doing, done).

### Implementation details

- New file: `src/cmd/update.rs` — loads config/store, resolves issue (by ID or current), writes content string to `issue.path`.
- `src/cmd/mod.rs` — adds `pub mod update`.
- `src/main.rs` — adds `Update { id: Option<String> }` variant to `Commands` enum; reads stdin into `content: String` before calling `cmd::update::run(id.as_deref(), content)`.
- Tests: `tests/features/update.feature` (4 BDD scenarios), new `when` steps in `tests/steps/when.rs`, new `then` step `the last issue content contains {string}` in `tests/steps/then.rs`.
- Docs: added examples to `CLAUDE.md` (Issue Management section) and `README.md` (usage example block and command table).
