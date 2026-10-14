# Rust Piscine

My solutions to the [01-edu Rust Piscine](https://github.com/01-edu/public/tree/master/subjects) exercises.

This repository is for learning Rust through hands-on programming exercises, focusing on fundamental concepts such as ownership, borrowing, data structures, error handling, and algorithms.

## Project Structure

The repository is organized by quest, with each exercise implemented as an independent Cargo package.

```text
rust-piscine/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── quest-01/
│   ├── scalar/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   ├── division_and_remainder/
│   └── fibonacci2/
├── quest-02/
│   └── ...
└── quest-03/
    └── ...
```

All exercises are managed using a [Cargo Workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).

## Getting Started

### Prerequisites

Install Rust and Cargo using [rustup](https://rustup.rs/).

Verify the installation:

```bash
rustc --version
cargo --version
```

### Clone the Repository

```bash
git clone https://github.com/<username>/rust-piscine.git
cd rust-piscine
```

### Running Tests

Run tests for a specific exercise:

```bash
cargo test -p scalar
```

Run tests for all exercises:

```bash
cargo test --workspace
```

Check the code with Clippy:

```bash
cargo clippy --workspace
```

Format the code:

```bash
cargo fmt --all
```

## Progress

### Quest 01 — Rust Fundamentals

| Exercise | Status |
|----------|--------|
| [scalar](https://github.com/01-edu/public/tree/master/subjects/scalar) | ✅️ |
| [division_and_remainder](https://github.com/01-edu/public/tree/master/subjects/division_and_remainder) | ✅️ |
| [fibonacci2](https://github.com/01-edu/public/tree/master/subjects/fibonacci2) | ✅️ |
| [find_factorial](https://github.com/01-edu/public/tree/master/subjects/find_factorial) | ✅️ |
| [groceries](https://github.com/01-edu/public/tree/master/subjects/groceries) | ✅️ |
| [looping](https://github.com/01-edu/public/tree/master/subjects/looping) | ✅️ |
| [matrix_transposition](https://github.com/01-edu/public/tree/master/subjects/matrix_transposition) | ✅️ |
| [reverse_string](https://github.com/01-edu/public/tree/master/subjects/reverse_string) | ✅️ |
| [speed_transformation](https://github.com/01-edu/public/tree/master/subjects/speed_transformation) | ✅️ |
| [temperature_conv](https://github.com/01-edu/public/tree/master/subjects/temperature_conv) | ✅️ |
| [tuples_refs](https://github.com/01-edu/public/tree/master/subjects/tuples_refs) | ✅️ |

### Quest 02 — Ownership & Borrowing

| Exercise | Status |
|----------|--------|
| [arrange_it](https://github.com/01-edu/public/tree/master/subjects/arrange_it) | ✅️ |
| [borrow](https://github.com/01-edu/public/tree/master/subjects/borrow) | ✅️ |
| [borrow_me_the_reference](https://github.com/01-edu/public/tree/master/subjects/borrow_me_the_reference) | ✅️ |
| [changes](https://github.com/01-edu/public/tree/master/subjects/changes) | ✅️ |
| [copy](https://github.com/01-edu/public/tree/master/subjects/copy) | ✅️ |
| [doubtful](https://github.com/01-edu/public/tree/master/subjects/doubtful) | ✅️ |
| [name_initials](https://github.com/01-edu/public/tree/master/subjects/name_initials) | ✅️ |
| [ownership](https://github.com/01-edu/public/tree/master/subjects/ownership) | ✅️ |
| [string_literals](https://github.com/01-edu/public/tree/master/subjects/string_literals) | ⬜ |
| [tic_tac_toe](https://github.com/01-edu/public/tree/master/subjects/tic_tac_toe) | ✅️ |

### Quest 03 — Structs, Enums & Collections

| Exercise | Status |
|----------|--------|
| [circle](https://github.com/01-edu/public/tree/master/subjects/circle) | ✅️ |
| [card_deck](https://github.com/01-edu/public/tree/master/subjects/card_deck) | ✅️ |
| [arrays](https://github.com/01-edu/public/tree/master/subjects/arrays) | ✅️ |
| [strings](https://github.com/01-edu/public/tree/master/subjects/strings) | ✅️ |
| [edit_distance](https://github.com/01-edu/public/tree/master/subjects/edit_distance) | ⬜ |
| [to_url](https://github.com/01-edu/public/tree/master/subjects/to_url) | ✅️ |
| [capitalizing](https://github.com/01-edu/public/tree/master/subjects/capitalizing) | ✅️ |
| [hashing](https://github.com/01-edu/public/tree/master/subjects/hashing) | ✅️ |
| [string_permutation](https://github.com/01-edu/public/tree/master/subjects/string_permutation) | ✅️ |
| [bigger](https://github.com/01-edu/public/tree/master/subjects/bigger) | ✅️ |
| [simple_hash](https://github.com/01-edu/public/tree/master/subjects/simple_hash) | ✅️ |
| [collect](https://github.com/01-edu/public/tree/master/subjects/collect) | ✅️ |

### Quest 04 — Error Handling

| Exercise | Status |
|----------|--------|
| [unwrap_or_expect](https://github.com/01-edu/public/tree/master/subjects/unwrap_or_expect) | ✅️ |
| [panic](https://github.com/01-edu/public/tree/master/subjects/panic) | ✅️ |
| [handling](https://github.com/01-edu/public/tree/master/subjects/handling) | ✅️ |
| [profanity_filter](https://github.com/01-edu/public/tree/master/subjects/profanity_filter) | ✅️ |
| [question_mark](https://github.com/01-edu/public/tree/master/subjects/question_mark) | ✅️ |
| [banner](https://github.com/01-edu/public/tree/master/subjects/banner) | ✅️ |
| [cipher](https://github.com/01-edu/public/tree/master/subjects/cipher) | ✅️ |
| [error_types](https://github.com/01-edu/public/tree/master/subjects/error_types) | ✅️ |
| [boxing_todo](https://github.com/01-edu/public/tree/master/subjects/boxing_todo) | ✅️ |

### Quest 05 — Advanced Types & Macros

| Exercise | Status |
|----------|--------|
| [middle_day](https://github.com/01-edu/public/tree/master/subjects/middle_day) | ⬜ |
| [does_it_fit](https://github.com/01-edu/public/tree/master/subjects/does_it_fit) | ⬜ |
| [macro_calculator](https://github.com/01-edu/public/tree/master/subjects/macro_calculator) | ⬜ |
| [shopping_mall](https://github.com/01-edu/public/tree/master/subjects/shopping_mall) | ⬜ |
| [expected_variable](https://github.com/01-edu/public/tree/master/subjects/expected_variable) | ⬜ |
| [mobs](https://github.com/01-edu/public/tree/master/subjects/mobs) | ⬜ |
| [invert_sentence](https://github.com/01-edu/public/tree/master/subjects/invert_sentence) | ⬜ |

Additional quests will be added as I progress.

## Development

Each exercise is implemented independently following the corresponding instructions in the official repository.

To create a new exercise:

```bash
cargo new --lib quest-01/scalar
```

Implement the solution in `src/lib.rs` and run the tests:

```bash
cargo test -p scalar
```

## References

- [01-edu Public Repository](https://github.com/01-edu/public)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)