# TermFlow

A hyper-productive, keyboard-driven **Terminal User Interface (TUI)** for managing your day-to-day tasks directly from the command line. TermFlow is written in Rust and powered by the excellent [`ratatui`](https://github.com/ratatui-org/ratatui) + [`crossterm`](https://github.com/crossterm-rs/crossterm) stack, giving you a fast and visually appealing experience that works on all major platforms.

---

## ✨ Key Features

* **Minimal yet powerful TUI** – Navigate with Vim-style keys, all information fits into a single screen.
* **Quick add** – Hit `n` and start typing to capture a new task in seconds.
* **Multiple task states** – Cycle between *Todo → In-Progress → Done* with a single <kbd>Space</kbd>.
* **Priorities** – Low / Medium / High (sorting favours high-priority items).
* **Rich categories** – Five built-ins (Personal, Work, Learning, Health, Finance) plus unlimited **custom categories** with your own emoji and colour.
* **Live search** – Press `/` and filter tasks instantly as you type.
* **Progress tracking** – Animated gauge shows completion percentage; separate today-only stats.
* **Transient notifications** – Small toast-style messages for actions like "Task deleted".
* **Zero-config** – Runs as a single binary; currently stores data in-memory only (persistence is planned in `storage/`).

---

## 📸 Screen Shot

![TermFlow Screenshot](images/image.png)


---

## ⌨️  Keyboard Shortcuts

| Mode&nbsp;\&nbsp;Key | `q` | `n` | `d` | `/` | <kbd>Space</kbd> | `↑` / `k` | `↓` / `j` | `Tab` | `Enter` | `Esc` | `Backspace` |
|--------------------|-----|-----|-----|-----|---------------|-----------|-----------|--------|---------|-------|--------------|
| **Normal**         | Quit | New task → *Insert* | Delete selected | Search → *Search* | Toggle state | Move up | Move down | – | – | – | – |
| **Insert**         | – | – | – | – | – | – | – | Choose category → *SelectCategory* | Save task | Cancel | Delete char |
| **SelectCategory** | – | – | – | – | – | – | – | Cycle categories | Confirm / or Create new | Back to *Insert* | – |
| **CreateCategory** | – | – | – | – | – | – | – | Cycle emoji / colour | Next step / Finish | Cancel | Delete char (step 1) |
| **Search**         | – | – | – | Exit search | – | – | – | – | Finish search | Cancel | Delete char |

> Tip: In *CreateCategory* step 3 you can press digits **1-7** to pick a colour directly.

---

## 🚀 Getting Started

### Prerequisites

* [Rust](https://rust-lang.org) toolchain **1.70+** (install via [`rustup`](https://rustup.rs))

### Build & Run

```bash
# Clone the repository
$ git clone https://github.com/yourname/termflow.git
$ cd termflow/termflow

# Run in debug mode
$ cargo run

# Or build an optimised binary
$ cargo build --release
$ ./target/release/termflow
```

The first launch opens TermFlow in full-screen **alternate screen** mode. Simply press `q` at any time to exit.

---

## 🗂️  Project Layout

```
termflow/
├── src/
│   ├── main.rs          # Application entry & event loop
│   ├── app.rs           # Core state machine & business logic
│   ├── ui/              # All rendering code (559 loc)
│   ├── models/          # Domain models (Task, Priority, Category…)
│   ├── storage/         # 🚧 Persistence layer (stub)
│   └── utils/           # Misc helpers (currently empty)
└── Cargo.toml           # Crate manifest
```

A quick overview of the main modules:

| Module | Responsibility |
|--------|----------------|
| `main.rs` | Sets up the terminal backend, drives the render loop, translates raw `crossterm` events into high-level actions. |
| `app.rs` | Central **state store**; exposes pure functions for mutating the state (add/delete/search tasks, category management, etc.). |
| `models/` | Strongly-typed domain entities. `Task` uses `uuid` + `chrono` for IDs & timestamps and `serde` for future persistence. |
| `ui/` | Declarative layout using `ratatui` widgets. Splits screen into header, task list, and status bar + pop-ups. |
| `storage/` | Placeholder for loading/saving tasks (JSON, SQLite, or cloud sync – contributions welcome!). |

---

## 🤝 Contributing

1. Fork the repo & create your branch: `git checkout -b feature/awesome`  
2. Commit your changes: `git commit -am 'Add awesome feature'`  
3. Push to the branch: `git push origin feature/awesome`  
4. Open a Pull Request !

Please make sure to format your code with `rustfmt` and run `cargo clippy` before submitting.

---

## 📄 License

This project is released under the **MIT License** – see [LICENSE](LICENSE) for details. 