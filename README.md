# tic_tac_toe

A command-line Tic-Tac-Toe game written in Rust.

This project demonstrates structured state management, enum-based domain modeling, and safe mutation patterns using Rust's ownership system.

---

## Overview

The game implements a 3×3 board with two players (X and O), turn-based gameplay, win detection (rows, columns, diagonals), draw detection, input validation, and deterministic state transitions.

The architecture separates three core concerns:

- **Board** — spatial state
- **GameState** — game progression
- **GameStatus** — termination state

---

## Features

- Strongly typed cell states using enums
- Explicit game state transitions
- Safe mutation through `&mut self`
- Input validation with retry loop
- Clear CLI rendering

---

## Project Structure

```
tic_tac_toe/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── board.rs
    ├── game.rs
    └── input.rs
```

| File | Responsibility |
|------|----------------|
| `board.rs` | Board representation and rendering |
| `game.rs` | Game logic and state transitions |
| `input.rs` | CLI input handling |
| `main.rs` | Application orchestration |

---

## How to Run

```bash
cargo run
```

To build a release binary:

```bash
cargo build --release
```

---

## How to Play

1. The board is displayed in the terminal.
2. Players take turns entering a position.
3. Input must be a valid empty cell.
4. The game ends when a player completes a row, column, or diagonal, or all cells are filled (draw).

---

## Design Notes

### Domain Modeling

Cell states are represented as an enum with three variants: `Empty`, `X`, and `O`. Game progression is tracked via `GameStatus::InProgress`, `GameStatus::Won(Player)`, and `GameStatus::Draw`. This ensures invalid states are impossible at the type level.

### State Transitions

Each move follows a strict sequence: validate input → ensure game is still active → mutate board → evaluate win condition → evaluate draw condition → switch player if appropriate. All mutations occur inside a single method to preserve invariants.

---

## Possible Improvements

- Implement `Display` for cleaner rendering
- Add AI opponent (minimax)
- Add unit tests for win detection
- Add replay option
- Improve board UI with separators and indexing

---

## Purpose

This project serves as a Rust learning exercise, a demonstration of ownership-aware design, and a small example of finite-state machine modeling.
