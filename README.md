# akartui-rs

A Rust-based terminal user interface (TUI) implementation of the **Akari (Light Up)** logic puzzle.

This project uses [ratatui](https://github.com/ratatui-org/ratatui) for rendering the UI, and provides a playable and interactive Akari puzzle game with archive support and keyboard navigation.

---

## Features

- Interactive TUI-based Akari game
- Archive of 750 puzzles (from Janko.at)
- Fully keyboard-controlled interface
- Dynamic board layout
- Puzzle metadata display
- Game states with lightbulbs, flags, and walls
- Random puzzle selection and archive browsing

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.70 or later recommended)
- Terminal with UTF-8 support

### Clone and Run

```bash
git clone https://github.com/G36maid/akartui-rs.git
cd akartui-rs
cargo run
````

---

## Controls

| Key     | Action                              |
| ------- | ----------------------------------- |
| ↑ ↓ ← → | Move cursor or menu navigation      |
| Enter   | Select item / Start puzzle          |
| Space   | Place or remove a lightbulb         |
| P       | Place or remove a flag              |
| Q       | Go back / Return to main menu       |
| G       | Start a new random puzzle (in menu) |

---

## App Layout

### Overall Structure

```
+--------------------------------------------------------+
| Puzzle Info (ID, Type, Author, Size, Source, Notes...) |
+----------------------+---------------------------------+
| Archive List (1~750) | Puzzle View / Menu / Settings   |
|                      |                                 |
+----------------------+---------------------------------+
| Controls Hint Bar (contextual)                         |
+--------------------------------------------------------+
```

### Screens

* **Main Menu**

  * New Game
  * Archive
  * Settings (TBD)
  * Help
  * Exit

* **Archive**

  * Scrollable list of 750 puzzles
  * Start any puzzle by pressing Enter

* **Game View**

  * Display puzzle board with dynamic layout
  * Place/remove bulbs and flags
  * Realtime light propagation logic

* **Settings**

  * Placeholder for future configuration

* **Help**

  * Keybindings and instructions

---

## Puzzle Data

Each puzzle contains:

* A dynamic board with:

  * **Walls** (`#`)
  * **Clue Walls** (`0~4`)
  * **Empty Cells** (`.`)
* Puzzle ID: 1–750
* Stored in `archive/` as individual text files
* Source: [Janko.at Akari Collection](https://www.janko.at/Raetsel/Akari/)

---

## Roadmap

* [x] Archive browser and puzzle loader
* [x] Main menu and random puzzle support
* [x] Game board display and cursor controls
* [ ] Settings screen
* [ ] Puzzle metadata/preview in archive
* [ ] Full win condition checks and hints

---

## License and Puzzle Sources

Puzzle data is based on the [Janko.at Akari Collection](https://www.janko.at/Raetsel/Akari/). Many puzzles were created by the Janko team and are published under:

> Creative Commons 3.0: Attribution – Non-Commercial – Share Alike

Some puzzles are used with permission from original authors. If you are a rights holder and would like attribution or removal, please contact us.

---

## Screenshots

*(Coming soon)*

---

## Credits

* TUI library: [ratatui](https://github.com/ratatui-org/ratatui)
* Puzzles: [Janko.at](https://www.janko.at/Raetsel/Akari/)
