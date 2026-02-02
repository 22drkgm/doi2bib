# DOI to BibTeX Converter (Rust GUI)

A fast, native desktop application written in Rust that converts a list of DOIs (Digital Object Identifiers) into formatted BibTeX citations. 

It uses the official `doi.org` API to ensure 100% accurate citations (matching `doi2bib.org`).

## 🚀 Features
- **GUI Interface**: Built with `egui` for a clean, fast experience.
- **Bulk Processing**: Load a CSV file containing thousands of DOIs.
- **Safety**: Validates DOIs and skips empty lines automatically.
- **Concurrency**: Runs conversion in the background without freezing the UI.
- **Cross-Platform**: Runs on Linux, Windows, and macOS.

## 📦 Installation

### Option 1: Download App (Linux)
Go to the [Releases Page](../../releases) and download the latest executable.

### Option 2: Build from Source
You need [Rust installed](https://rustup.rs/).

```bash
git clone [https://github.com/22drkgm/doi2bib.git](https://github.com/22drkgm/doi2bib.git)
cd doi2bib
cargo run --release
