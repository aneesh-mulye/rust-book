# Repository Guidelines

## Project Structure & Module Organization

This repository follows the Rust Book chapter flow.

- `projects/` contains chapter-aligned standalone Cargo projects such as `projects/01_hello_world` and `projects/07_rectangles`.
- `challenges/` contains generated practice sets by chapter. Each chapter has tier crates like `challenges/chapter_06/tier_3_option`, plus a chapter-level `run_all_tier_tests.sh`.
- `one-offs/` is a scratch Cargo crate for experiments.
- `resources/` stores reference material and is not part of the build.

Within challenge crates, use `src/lib.rs` to expose modules and keep one file per challenge named `challenge_<tier>_<num>_<slug>.rs`.

## Build, Test, and Development Commands

- `cargo run --manifest-path projects/07_rectangles/Cargo.toml` runs a single book project.
- `cargo test --manifest-path one-offs/Cargo.toml` runs tests for the scratch crate.
- `cargo test` from inside a tier crate runs only that tier’s tests.
- `./challenges/chapter_06/run_all_tier_tests.sh --no-run` checks that every tier harness compiles.
- `./challenges/chapter_06/run_all_tier_tests.sh` runs all tests for that chapter.
- `cargo fmt --all` formats Rust code when run from a crate directory.

## Coding Style & Naming Conventions

Use default Rust style: 4-space indentation, `snake_case` for functions/modules, `CamelCase` for types and enums, and short explanatory comments only where needed. Prefer one concept per file. For generated challenge scaffolds, keep stub bodies unsolved and place tests below the `// .` spacer block so solutions are not visible at a glance.

## Testing Guidelines

Tests are standard Rust unit tests inside each challenge file under `#[cfg(test)]`. Name tests by behavior, for example `prompt_navigation_sequence_is_correct`. Keep failure messages descriptive enough to guide fixing the code. Run a single challenge with commands like `cargo test challenge_3_3_chained_optionality`.

## Commit & Pull Request Guidelines

Recent history uses short, imperative, chapter-focused commit messages such as `Add chapter 6 challenges` or `Finished Chapter 4 quizzing`. Follow that pattern: keep commits scoped and specific. For pull requests, include a brief summary, list affected chapters or crates, and note the verification command you ran. For challenge-generation changes, mention whether files are intentionally unsolved scaffolds.
