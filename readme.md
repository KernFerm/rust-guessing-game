<div align="center">
  <img src="https://github.com/KernFerm/rust-guessing-game/blob/main/banner/Guess_The_Number_Game-logo.png" alt="Guess The Number Game" width="500"/>
</div>

# ❓❓ Guess the Number Game ❓❓ 

Welcome to the Guess the Number game! 🎉 This is a fun and simple text-based game with an interactive interface where you try to guess a randomly generated number between 0 and 100. The game includes hints, limited attempts, and a user-friendly Text User Interface (TUI) to keep you engaged! 😄

# Join the discord if you need help 

[https://fnbubbles420.org/discordinvite](https://fnbubbles420.org/discordinvite)


## Support the Project ⭐

If you find this project useful, please give it a star! Your support is appreciated and helps keep the project growing. 🌟
-----

## 📜 Description

This project is a **Rust** implementation of the classic "Guess the Number" game. The program generates a random number between 0 and 100, and the player must guess it within 7 attempts. The game provides feedback on whether the guess is too high, too low, or correct. Players are also offered up to 3 hints to assist them during the game.

This program was built and tested on a **64-bit Windows system** using the `pancurses` library for the interface.

## ✨ Features

- 🎲 **Random number generation** between 0 and 100.
- 🎮 **7 attempts** to guess the correct number.
- 💡 **Up to 3 optional hints** to help guide your guesses.
- 📊 **Feedback** on whether the guess is too high, too low, or correct.
- 🖥️ **User-friendly interface** built using the `pancurses` library.

# 📁 Project Structure

Here is the structure of the **Guess the Number Game** project:

- 📂 **.github/ISSUE_TEMPLATE**: Contains templates for issue reporting.
- 📂 **banner**: Includes project banners and images (e.g., game banners).
- 📂 **src**: Source code of the project.
- 📄 **.gitignore**: Defines which files and directories should be ignored by Git.
- 📜 **CODE_OF_CONDUCT.md**: Code of conduct guidelines for contributors.
- 🔒 **Cargo.lock**: Lock file for dependencies, automatically generated.
- 📄 **Cargo.toml**: Cargo configuration file containing project dependencies and settings.
- 📄 **LICENSE**: Custom license for the project detailing the conditions under which the software may be used, learned from, or modified.
- 📄 **readme.md**: The main README file with game instructions, setup, and information.

## 🛠️ Prerequisites

To run this project, you need:
- **Rust** and **Cargo** installed on your system.
- A **64-bit Windows** system (for compatibility with this build).

### Installing Rust and Cargo

If you don't have Rust and Cargo installed, follow these steps:

1. Go to the [Rust installation page](https://www.rust-lang.org/tools/install).
2. Follow the instructions to download and run the installer for your operating system.
3. After installation, you can verify that Rust and Cargo are installed by running the following commands in your terminal:
    ```
    rustc --version
    cargo --version
    ```

## 🚀 Installation

1. Clone or download the project files to your local machine:
    ```
    git clone https://github.com/kernferm/rust-guessing-game.git
    ```
2. Navigate to the project directory:
    ```
    cd rust-guessing-game
    ```

## 🕹️ Usage

To build and run the game, use the following commands:

1. Open a terminal and navigate to the project directory.
2. Build the project using Cargo:
    ```
    cargo build
    ```
3. Run the game:
    ```
    cargo run
    ```

### 🎮 Gameplay

- Once the game starts, follow the instructions on the screen.
- You have **7 attempts** to guess the number between 0 and 100.
- After each guess, the program will tell you if your guess is too high, too low, or correct.
- You have the option to use up to **3 hints** during the game to help narrow down the correct number.

## 🖥️ Built On

This game was built and tested on:
- **64-bit Windows 10** using Rust and the `pancurses` library.

## 🤝 Contributing

Contributions are welcome! If you'd like to contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes.
4. Commit your changes (`git commit -m 'Add some feature'`).
5. Push to the branch (`git push origin feature-branch`).
6. Open a pull request.

## 📄 License
### Learn-and-Use License

Copyright (c) 2024 Bubbles The Dev & FNBUBBLES420 ORG

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to use
and learn from the Software, subject to the following conditions:

1. **Usage**: The Software may be used for personal, educational, or non-commercial purposes only.
2. **Learning**: Users are permitted to study the code to learn from it.
3. **Modifications**: No modifications or derivative works of the Software are allowed without prior written consent from the original developer.
4. **Redistribution**: The Software may not be redistributed, sold, or sublicensed in any form without express permission from the original developer.
5. **Commercial Use**: The Software may not be used for commercial purposes without explicit written approval from the original developer.

### Conditions

- Any use of the Software that violates the terms of this license will result in revocation of permission to use, study, or learn from the Software.
- You are not permitted to remove or alter the copyright notice or this permission notice.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE, AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES, OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT, OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

