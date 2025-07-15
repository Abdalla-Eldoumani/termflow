# TermFlow

A **hyper-productive, keyboard-driven Terminal User Interface (TUI)** for managing your day-to-day tasks with advanced time management features. TermFlow combines task management with **Pomodoro Timer**, **Time Blocking**, and **Smart Analytics** - all directly from the command line. Built in Rust with [`ratatui`](https://github.com/ratatui-org/ratatui) + [`crossterm`](https://github.com/crossterm-rs/crossterm), delivering a fast, visually stunning experience across all platforms.

---

## 🚀 **What's New in TermFlow Enhanced**

### 🍅 **Advanced Pomodoro Timer System**
- **Full-featured timer** with work sessions (25min), short breaks (5min), and long breaks (15min)
- **Visual progress tracking** with animated gauges and real-time countdown
- **Smart session management** - automatically suggests next session type after completion
- **Motivational messages** that adapt to your session type and time of day
- **Session statistics** - tracks total sessions, focus time, and daily progress
- **Pause/Resume functionality** with accurate time tracking
- **Task integration** - start focused Pomodoro sessions for specific tasks

### ⏰ **Smart Time Blocking**
- **Flexible scheduling** - 25min (Pomodoro), 45min, 60min, or 90min time blocks
- **Task-specific scheduling** - assign time blocks to individual tasks
- **Upcoming schedule view** - see your planned time blocks at a glance
- **Seamless integration** with Pomodoro timer for optimal productivity flow

### 🎨 **Enhanced User Experience**
- **Animated welcome screen** for new users with feature showcase
- **Theme-aware styling** that adapts to your chosen visual theme
- **Professional timer interface** with progress indicators and statistics
- **Smooth transitions** and visual feedback throughout the application

---

## ✨ Core Features

* **Minimal yet powerful TUI** – Navigate with Vim-style keys, all information fits into a single screen.
* **Quick task creation** – Hit `n` and start typing to capture a new task in seconds.
* **Advanced task states** – Cycle between *Todo → In-Progress → Done* with <kbd>Space</kbd>.
* **Smart priorities** – Low / Medium / High with intelligent sorting.
* **Rich categories** – Five built-ins (Personal, Work, Learning, Health, Finance) plus unlimited **custom categories** with emojis and colors.
* **Live search** – Press `/` and filter tasks instantly as you type.
* **Comprehensive analytics** – Progress tracking, streak counters, and productivity insights.
* **Multiple themes** – Choose from Cyberpunk, Forest, Ocean, Sunset, and Midnight themes.
* **Data persistence** – Auto-save with JSON export and backup functionality.
* **Zero-config setup** – Runs as a single binary with intelligent defaults.

---

## 📸 Screen Shot

![TermFlow Screenshot](images/image.png)


---

## ⌨️  Keyboard Shortcuts

### 🎯 **Core Navigation**
| Key | Action | Description |
|-----|--------|-------------|
| `q` | Quit | Exit TermFlow |
| `↑` / `k` | Move up | Navigate task list upward |
| `↓` / `j` | Move down | Navigate task list downward |
| <kbd>Space</kbd> | Toggle state | Cycle task: Todo → In-Progress → Done |
| `Esc` | Cancel/Back | Return to previous mode |

### 📝 **Task Management**
| Key | Action | Description |
|-----|--------|-------------|
| `n` | New task | Create a new task |
| `d` | Delete | Delete selected task |
| `/` | Search | Filter tasks by search term |
| `e` | Export | Export data to JSON file |

### 🍅 **Pomodoro Timer** *(NEW!)*
| Key | Action | Description |
|-----|--------|-------------|
| `p` | Start Pomodoro | Begin 25-minute focus session for selected task |
| `b` | Start Break | Begin short (5min) or long (15min) break |
| <kbd>Space</kbd> | Pause/Resume | Pause or resume active timer |
| `s` | Stop Timer | Stop current timer session |

### ⏰ **Time Blocking** *(NEW!)*
| Key | Action | Description |
|-----|--------|-------------|
| `T` | Time Block | Open time blocking interface |
| `1` | 25 minutes | Schedule 25-minute Pomodoro block |
| `2` | 45 minutes | Schedule 45-minute deep focus block |
| `3` | 60 minutes | Schedule 1-hour extended work block |
| `4` | 90 minutes | Schedule 90-minute deep work marathon |

### 📊 **Analytics & Themes**
| Key | Action | Description |
|-----|--------|-------------|
| `s` | Statistics | View productivity dashboard and analytics |
| `t` | Themes | Cycle through visual themes |

### 🎨 **Task Creation & Categories**
| Mode | Key | Action |
|------|-----|--------|
| **Insert** | `Tab` | Choose category |
| **Insert** | `Enter` | Save task |
| **SelectCategory** | `Tab` | Cycle categories |
| **SelectCategory** | `Enter` | Confirm selection or create custom |
| **CreateCategory** | `Tab` | Cycle emoji/color options |
| **CreateCategory** | `1-7` | Quick color selection |

> 💡 **Pro Tips:** 
> - Press any key on the welcome screen to start using TermFlow
> - Use `p` on any task to immediately start a focused Pomodoro session
> - Time blocks automatically integrate with your Pomodoro workflow

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

## 🎯 **Productivity Features in Detail**

### 🍅 **Pomodoro Timer Workflow**
1. **Select a task** from your list using arrow keys
2. **Press `p`** to start a 25-minute focused work session
3. **Visual feedback** with countdown timer and progress bar
4. **Automatic break suggestions** when session completes
5. **Session tracking** - all your focus time is recorded and analyzed

### ⏰ **Time Blocking Made Simple**
1. **Press `T`** to open the time blocking interface
2. **Choose duration**: 25min (Pomodoro), 45min, 60min, or 90min
3. **Automatic scheduling** - blocks are scheduled 5 minutes from now
4. **Visual schedule** - see upcoming time blocks at a glance
5. **Seamless integration** with Pomodoro timer for optimal flow

### 📊 **Smart Analytics Dashboard**
- **Productivity heatmap** showing your most active days
- **Category breakdown** with visual progress bars
- **Streak tracking** with motivational messages
- **Focus time statistics** from Pomodoro sessions
- **Daily/weekly/monthly** productivity insights

### 🎨 **Visual Themes**
Choose from 5 beautiful themes that transform your entire experience:
- **Cyberpunk** - Neon blues and magentas for night owls
- **Forest** - Calming greens for natural focus
- **Ocean** - Deep blues for tranquil productivity
- **Sunset** - Warm oranges and reds for evening sessions
- **Midnight** - Dark elegance for late-night work

---

## 🗂️  Project Architecture

```
termflow/
├── src/
│   ├── main.rs          # Application entry & event loop with timer integration
│   ├── app.rs           # Enhanced state machine with Pomodoro & time blocking
│   ├── timer.rs         # 🍅 NEW: Pomodoro timer logic and session management
│   ├── ui/              # Comprehensive UI with welcome screen & timer interfaces
│   ├── models/          # Extended domain models with time management features
│   ├── storage/         # JSON persistence with auto-save and backup
│   ├── theme.rs         # Multi-theme system with 5 visual styles
│   └── utils/           # Helper utilities
└── Cargo.toml           # Enhanced dependencies for audio & notifications
```

### 🏗️ **Enhanced Module Overview**

| Module | Responsibility | Key Features |
|--------|----------------|--------------|
| `main.rs` | Terminal backend, event loop, input handling | Timer updates, welcome screen dismissal, enhanced key bindings |
| `app.rs` | Central state management & business logic | Pomodoro integration, time blocking, welcome screen animation |
| `timer.rs` | **NEW** - Pomodoro timer system | Session management, progress tracking, motivational messages |
| `models/` | Domain entities with time management | Task with Pomodoro sessions, time blocks, recurring patterns |
| `ui/` | Declarative UI with advanced interfaces | Welcome screen, timer UI, time blocking interface, enhanced analytics |
| `storage/` | Data persistence with JSON export | Auto-save, backup system, statistics tracking |
| `theme.rs` | Multi-theme visual system | 5 themes with consistent color schemes |

---

## 🚀 **Quick Start Guide**

### First Time Users
1. **Launch TermFlow** - `cargo run` or `./target/release/termflow`
2. **Welcome screen** appears with animated introduction
3. **Press any key** to dismiss welcome and start using TermFlow
4. **Create your first task** - Press `n` and type your task
5. **Start a Pomodoro** - Select task and press `p` for focused work

### Power User Workflow
```bash
# Morning routine
n → "Review project requirements" → Enter    # Create task
p → [25min focus session] → b → [5min break] # Pomodoro cycle
T → 2 → [Schedule 45min deep work block]     # Time blocking
s → [Check productivity stats]               # Analytics review
t → [Switch to Forest theme for focus]       # Theme change
```

### Daily Productivity Flow
1. **Morning**: Review tasks, set time blocks for the day
2. **Work sessions**: Use Pomodoro timer for focused work
3. **Breaks**: Automatic break suggestions between sessions
4. **Evening**: Check statistics dashboard for insights
5. **Themes**: Switch themes based on time of day or mood

---

## 🌟 **Why TermFlow?**

### **Terminal-Native Productivity**
Unlike web-based or GUI applications, TermFlow runs entirely in your terminal, making it:
- **Lightning fast** - No browser overhead or GUI bloat
- **Keyboard-driven** - Never touch your mouse again
- **Distraction-free** - Clean, focused interface without notifications
- **Universal** - Works on any system with a terminal

### **Advanced Time Management**
TermFlow goes beyond simple task lists:
- **Pomodoro Integration** - Built-in timer with session tracking
- **Time Blocking** - Schedule focused work periods
- **Smart Analytics** - Understand your productivity patterns
- **Streak Tracking** - Maintain momentum with daily goals

### **Professional Features**
- **Data Persistence** - Your tasks and statistics are automatically saved
- **Export Functionality** - JSON export for backup and analysis
- **Multiple Themes** - Customize your visual experience
- **Custom Categories** - Organize tasks your way with emojis and colors

---

## 📊 **Feature Comparison**

| Feature | TermFlow | Traditional Task Apps | Web-based Tools |
|---------|----------|----------------------|-----------------|
| **Terminal Native** | ✅ | ❌ | ❌ |
| **Pomodoro Timer** | ✅ | ❌ | Some |
| **Time Blocking** | ✅ | ❌ | Limited |
| **Offline First** | ✅ | Varies | ❌ |
| **Keyboard Shortcuts** | ✅ Full | Limited | Limited |
| **Custom Themes** | ✅ 5 themes | ❌ | Limited |
| **Analytics Dashboard** | ✅ | Basic | ✅ |
| **Zero Setup** | ✅ | ❌ | ❌ |
| **Privacy** | ✅ Local | Varies | ❌ Cloud |
| **Speed** | ⚡ Instant | Slow | Depends on connection |

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