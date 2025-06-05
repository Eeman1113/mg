
# mg - worlds fastest language corrector using rust

[](https://opensource.org/licenses/MIT)
[](https://www.rust-lang.org/)

`mg` is a high-speed, command-line tool written in Rust that helps you improve your writing. It parses text files to detect and highlight common stylistic issues, such as weasel words, passive voice, duplicate words, and spelling errors.
![Screenshot 2025-06-05 at 7 04 20 PM](https://github.com/user-attachments/assets/7e912f58-33d4-4b51-816b-08732a81a848)

## ✨ Features

  - **Weasel Word Detection**: Flags words that are ambiguous and weaken your statements (e.g., "many", "extremely", "very").
  - **Passive Voice Detection**: Identifies sentences written in the passive voice, helping you write in a more direct and active style.
  - **Duplicate Word Check**: Catches accidental word repetitions (e.g., "the the").
  - **Spelling Check**: Points out common spelling mistakes.
  - **Colored Output**: Uses terminal colors to clearly distinguish suggestions and errors.

## ⚙️ Installation

To use `mg`, you need to have Rust and Cargo installed on your system. If you don't have them, you can install them from [rustup.rs](https://rustup.rs/).

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/Eeman1113/mg.git
    cd mg
    ```

2.  **Build the project:**
    For the best performance, build the project in release mode.

    ```bash
    cargo build --release
    ```

3.  **Install the binary (Optional):**
    To make the `writing-checker` binary available anywhere in your system, you can install it using Cargo:

    ```bash
    cargo install --path .
    ```

## 🚀 Usage

The tool is run from your terminal. The binary name is `writing-checker`.

If you installed it using `cargo install`, you can run it directly:

```bash
writing-checker <path/to/your/file.txt>
```

If you built it with `cargo build`, you can run it through Cargo:

```bash
cargo run --release -- <path/to/your/file.txt>
```

### Example

Let's say you have a file named `mydoc.txt` with the following content:

```
This is a very interesting document. A solution was discovered by the the team. Many people agree that this is quite significant.
```

Run the checker on this file:

```bash
writing-checker mydoc.txt
```

The output will look something like this, with colors highlighting the issues:

```markdown
---------------------
PASSIVE VOICE
---------------------
A solution was discovered by the the team.

---------------------
WEASEL WORDS
---------------------
This is a very interesting document.
Many people agree that this is quite significant.

---------------------
DUPLICATE WORDS
---------------------
A solution was discovered by the the team.

```

## 🤝 Contributing

Contributions are welcome\! If you have suggestions for new features, improvements, or bug fixes, feel free to open an issue or submit a pull request.

1.  Fork the repository.
2.  Create your feature branch (`git checkout -b feature/AmazingFeature`).
3.  Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4.  Push to the branch (`git push origin feature/AmazingFeature`).
5.  Open a Pull Request.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](https://www.google.com/search?q=LICENSE) file for more details.

-----
