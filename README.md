# stowr

**stowr** is a modular, Rust‐powered asset management system with three distinct
user interfaces (CLI, TUI, and GUI) and a shared core domain model. Whether you
prefer a lightweight command‐line tool, a rich terminal interface, or a modern
web/desktop GUI (via Dioxus), stowr has you covered. It’s designed for
flexibility, strong type safety, and clean separation of concerns.

---

## 🛠️ Getting Started

### Prerequisites

- **Rust & Cargo** (≥ 1.70.0 recommended)
- **Dioxus CLI** (`dioxus-cli`) is required for running or building the GUI frontend

```bash
# Verify Rust toolchain
rustup show
rustc --version

# Install Dioxus CLI if you haven't already
cargo install dioxus-cli

# Verify Dioxus
dioxus --version
```

### Clone & Build Core, CLI, and TUI

```bash
git clone https://github.com/your-username/stowr.git
cd stowr

# Build core, CLI, and TUI (all Rust crates) at once
cargo build --workspace --release
```

This will produce two runnable binaries under `target/release`:

- `stowr-cli` (the CLI frontend)
- `stowr-tui` (the TUI frontend)

> **Note:** The GUI frontend is not built via `cargo build`. See the
> [GUI section](crates/gui/README.md) for details on building and running.

---

## 📦 Installation

You can install the CLI and TUI frontends globally via Cargo:

```bash
# Install CLI frontend
cargo install --path crates/cli

# Install TUI frontend
cargo install --path crates/tui
```

> **Tip:** After installing, `stowr-cli` and `stowr-tui` will be available in
> your `$PATH`. **Note:** The GUI cannot be installed via Cargo; it relies on
> Dioxus tooling (see below).

---

## ⚙️ Configuration

Currently, **stowr** does not require an external database. All data operations
are in-memory or via simple file storage. In the future, a SurrealDB local-mode
instance will be provided by the core library, allowing all frontends to
communicate with a lightweight, embedded database.

---

## 💡 Usage

Usage instructions for each component are maintained in their respective crate
directories. For detailed examples and command references, please consult the
following:

- **Core:** [`crates/core/README.md`](crates/core/README.md)
- **CLI:** [`crates/cli/README.md`](crates/cli/README.md)
- **TUI:** [`crates/tui/README.md`](crates/tui/README.md)
- **GUI:** [`crates/gui/README.md`](crates/gui/README.md)

---

## 🤝 Contributing

Contributions are more than welcome! Whether you want to fix a typo, add a brand‑new feature, or file an issue, please follow these guidelines:

1. **Clone & Fork**

   ```bash
   git clone https://github.com/your-username/stowr.git
   cd stowr
   git remote add upstream https://github.com/original-owner/stowr.git
   ```

2. **Create a Branch**

   ```bash
   git checkout -b feature/awesome-new-thing
   ```

3. **Run Tests & Linters**

   ```bash
   # From workspace root:
   cargo fmt -- --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test
   ```

4. **Commit & Push**

   ```bash
   git add .
   git commit -m "Add awesome new feature"
   git push origin feature/awesome-new-thing
   ```

5. **Open a Pull Request** against `main`.

- Use descriptive titles and link any relevant issues.
- Include before/after screenshots or GIFs if you modify the UI.

---

## 📜 License

This project is licensed under the **MIT License**. See [LICENSE](LICENSE) for
full details.

---

## 🔖 Acknowledgments

- Inspired by the mature open‑source projects
  [homebox](https://github.com/sysadminsmedia/homebox) and
  [grocy](https://github.com/grocy/grocy); please check them out for more
  feature‑complete asset management solutions. Stowr is currently focused on a
  narrower set of use cases tailored to specific needs.
- Additional terminal UI examples borrowed from the [Ratatui
  cookbook](https://github.com/tui-rs/examples).
- GUI design courtesy of the Dioxus starters template.
- Thanks to all contributors who help make stowr better every day!

---

> “Because no one ever says, ‘I love digging through boxes.’”
> — _The stowr Team_
