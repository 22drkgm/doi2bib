# DOI to BibTeX Converter (Rust GUI)

![App Screenshot](screenshot.png)

A fast, native desktop application written in Rust that converts a list of DOIs (Digital Object Identifiers) into formatted BibTeX citations. 

It uses the official `doi.org` API to ensure 100% accurate citations (matching `doi2bib.org`).

## 🚀 Features
- **GUI Interface**: Built with `egui` for a clean, fast experience.
- **Bulk Processing**: Load a CSV file containing thousands of DOIs.
- **Custom Output**: Choose exactly where to save your `references.bib` file.
- **Safety**: Validates DOIs and skips empty lines automatically.
- **Concurrency**: Runs conversion in the background without freezing the UI.

## 📖 How to Use
1. **Prepare your Data**: Create a `.csv` file with your DOIs listed in the first column. (Headers are automatically ignored).
2. **Select Input**: Click **📂 Input CSV** and choose your file.
3. **Select Output**: (Optional) Click **Dt Output Folder** to choose where to save the result. If skipped, it saves next to your input file.
4. **Run**: Click **🚀 Start Conversion**.
5. **Done**: Check the logs for `✔ OK` messages and open your new `references.bib` file!

## 📦 Installation

### Option 1: Download App (Linux)
Go to the [Releases Page](../../releases) and download the latest executable (`doi2bib`).
Then, give it permission to run:
```bash
chmod +x doi2bib
./doi2bib
```
# Option 2: Build from Source
## You need Rust installed.

```Bash
git clone [https://github.com/22drkgm/doi2bib.git](https://github.com/22drkgm/doi2bib.git)
cd doi2bib
cargo run --release
```

#  Call for Contributors (Windows / macOS / Android)
We need your help! This application is built with Rust and egui, making it natively cross-platform. Currently, binaries are only provided for Linux.

If you are on Windows, macOS, or Android, please fork this repository!

Fork the repo.

Compile the project on your machine (cargo build --release).

Submit a Pull Request or open an Issue so we can add support for your platform!
