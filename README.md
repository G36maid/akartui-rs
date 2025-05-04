# akartui-rs

A Rust-based terminal user interface (TUI) implementation of the **Akari (Light Up)** logic puzzle.

This project uses [ratatui](https://github.com/ratatui-org/ratatui) for rendering the UI, and provides a playable and interactive Akari puzzle game with archive support and keyboard navigation.

---

## Screenshots

![圖片](https://github.com/user-attachments/assets/afafd3dc-1a03-46b7-90f8-e196922529bd)


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
## Game Rules
To illustrate the rules, here is an example using puzzle 001:

**Puzzle 001 (problem):**

```
1 - - - - - - - - 1
- - - x - - - - - -
- x - - - 2 - - x -
- - - - - - - 1 - -
- - - 4 - - - - - -
- - - - - - 2 - - -
- - 2 - - - - - - -
- x - - 2 - - - x -
- - - - - - 0 - - -
1 - - - - - - - - 1
```

- `1`, `2`, `4`, `0` are clue black cells (numbered walls).
- `x` is a black cell (wall) with no clue.
- `-` is an empty cell where bulbs can be placed.

**Solution:**

```
1 - - - - - - - o 1
o - - x - o - - - -
- x - - - 2 o - x -
- - - o - - - 1 o -
- - o 4 o - - - - -
- - - o - - 2 o - -
- o 2 - - - o - - -
- x o - 2 o - - x -
- - - - o - 0 - - o
1 o - - - - - - - 1
```

- `o` indicates a placed lightbulb.

**How the rules apply:**
- Every empty cell is lit by at least one bulb.
- No two bulbs shine on each other in the same row or column.
- Numbered black cells have exactly that many bulbs adjacent (orthogonally).
- Black cells with `0` have no bulbs adjacent.
- Unnumbered black cells (`x`) can have any number of bulbs adjacent.

This example demonstrates all the main rules of Akari (Light Up).


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

  * Light Up is played on a rectangular grid of white and black cells. The player places light bulbs in white cells such that no two bulbs shine on each other, until the entire grid is lit up. A bulb sends rays of light horizontally and vertically, illuminating its entire row and column unless its light is blocked by a black cell. A black cell may have a number on it from 0 to 4, indicating how many bulbs must be placed adjacent to its four sides; for example, a cell with a 4 must have four bulbs around it, one on each side, and a cell with a 0 cannot have a bulb next to any of its sides. An unnumbered black cell may have any number of light bulbs adjacent to it, or none. Bulbs placed diagonally adjacent to a numbered cell do not contribute to the bulb count.


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



---

## Credits

* TUI library: [ratatui](https://github.com/ratatui-org/ratatui)
* Puzzles: [Janko.at](https://www.janko.at/Raetsel/Akari/)
