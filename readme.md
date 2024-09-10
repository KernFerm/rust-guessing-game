# 🎯 Guess the Number Game

Welcome to the **Guess the Number** game! 🎉 This is a fun and simple command-line game where you try to guess a randomly generated number between 0 and 100. The game includes hints, limited attempts, and a user-friendly interface to keep you engaged! 😄

## 📜 Description

This project is a **Rust** implementation of the classic "Guess the Number" game. The program generates a random number between 0 and 100, and the player must guess it within 7 attempts. The game provides feedback on whether the guess is too high, too low, or correct. Players are also offered up to 3 hints to assist them during the game.

This program was built and tested on a **64-bit Windows system** using the `pancurses` library for the interface.

## ✨ Features

- 🎲 **Random number generation** between 0 and 100.
- 🎮 **7 attempts** to guess the correct number.
- 💡 **Up to 3 optional hints** to help guide your guesses.
- 📊 **Feedback** on whether the guess is too high, too low, or correct.
- 🖥️ **User-friendly interface** built using the `pancurses` library.

## 🛠️ Prerequisites

To run this project, you need:
- **Rust** and **Cargo** installed on your system.
- A **64-bit Windows** system (for compatibility with this build).

### Installing Rust and Cargo

If you don't have Rust and Cargo installed, follow these steps:

1. Go to the [Rust installation page](https://www.rust-lang.org/tools/install).
2. Follow the instructions to download and run the installer for your operating system.
3. After installation, you can verify that Rust and Cargo are installed by running the following commands in your terminal:
    ```sh
    rustc --version
    cargo --version
    ```

## 🚀 Installation

1. Clone or download the project files to your local machine:
    ```sh
    git clone https://github.com/kernferm/guess-the-number-game.git
    ```
2. Navigate to the project directory:
    ```sh
    cd guess-the-number-game
    ```

## 🕹️ Usage

To build and run the game, use the following commands:

1. Open a terminal and navigate to the project directory.
2. Build the project using Cargo:
    ```sh
    cargo build
    ```
3. Run the game:
    ```sh
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

This project is licensed under the MIT License. See the LICENSE file for more details.
