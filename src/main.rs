use pancurses::{initscr, endwin, Input, resize_term, Window};
use rand::Rng;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let window = initscr();

    // Set the window to a medium size
    let width = 80;  // Medium width
    let height = 24; // Medium height

    resize_term(height, width);
    window.clear();
    window.printw("This is the guessing game. You have 3 hints and must guess a number between 0 and 100 within 7 attempts.\n");
    window.printw("Press 's' to start the game.\n");
    window.refresh();

    // Wait for the user to press the 's' key to start the game
    loop {
        match window.getch() {
            Some(Input::Character('s')) => {
                window.clear();
                window.printw("Game is starting...\n");
                window.refresh();
                sleep(Duration::from_secs(1)); // Optional delay before starting
                break;
            }
            _ => {} // Ignore other inputs
        }
    }

    // Game starts
    let secret_number = rand::thread_rng().gen_range(0..=100);
    let mut attempts = 7;
    let mut hints = 3;
    let mut hint_level = 0;

    while attempts > 0 {
        window.clear(); // Clear the screen for a fresh display
        window.printw("Rate / Guess (0-100): ");
        window.refresh();

        let guess = get_user_input(&window);

        if guess == -1 {
            window.printw("Invalid input. Please enter a number between 0 and 100.\n");
            sleep(Duration::from_secs(2));
            continue;
        }

        if guess == secret_number {
            window.printw("Correct, you have won!\n");
            window.refresh();
            break;  // Win condition met
        } else if guess > 100 || guess < 0 {
            window.printw("This number is not between 0 and 100!\n");
        } else {
            attempts -= 1;
            if attempts == 0 {
                window.printw(&format!("You lost. The correct number was {}\n", secret_number));
                window.refresh();
                break;  // Lose condition met
            } else {
                window.printw(&format!("Wrong! {} attempts remaining.\n", attempts));
            }

            if hints > 0 {
                window.printw("Do you want a hint? (y/n): ");
                window.refresh();

                let mut user_response = None;
                let start_time = Instant::now();

                // Loop to wait for "y" or "n" input within 30 seconds
                while start_time.elapsed() < Duration::from_secs(30) {
                    match window.getch() {
                        Some(Input::Character('y')) | Some(Input::Character('Y')) => {
                            user_response = Some("yes");
                            break;
                        }
                        Some(Input::Character('n')) | Some(Input::Character('N')) => {
                            user_response = Some("no");
                            break;
                        }
                        _ => {} // Ignore other inputs
                    }
                    sleep(Duration::from_millis(100)); // Small delay to avoid busy-waiting
                }

                if let Some(response) = user_response {
                    if response == "yes" {
                        hints -= 1;
                        window.printw("Displaying hint...\n");
                        give_hint(secret_number, hint_level, &window);
                        hint_level += 1;
                    } else {
                        window.printw("You chose not to use a hint. Continuing...\n");
                    }
                } else {
                    window.printw("No response in 30 seconds. Continuing without a hint.\n");
                }
            }
        }
        window.refresh();
        sleep(Duration::from_secs(2));  // Sleep to allow user to read messages
    }

    // Pause before closing the game window
    window.printw("\nPress any key to exit.");
    window.refresh();
    window.getch();  // Wait for a key press before exiting
    endwin();
}

fn get_user_input(window: &Window) -> i32 {
    let mut input = String::new();

    // Capture input until Enter is pressed
    loop {
        match window.getch() {
            Some(Input::Character(c)) if c.is_digit(10) => {
                input.push(c); // Append the digit to the input string
            }
            Some(Input::Character('\n')) => {
                break; // Enter key ends input
            }
            _ => {} // Ignore non-digit inputs
        }
    }

    input.parse().unwrap_or(-1) // Return -1 if parsing fails or invalid input
}

fn give_hint(number: i32, hint_level: i32, window: &Window) {
    match hint_level {
        0 => {
            if number < 50 {
                window.printw("The number is less than 50\n");
            } else {
                window.printw("The number is greater than 50\n");
            }
        }
        1 => {
            if number > 5 && number < 80 {
                window.printw("The number is between 5 and 80\n");
            } else {
                window.printw("The number is not between 5 and 80\n");
            }
        }
        2 => {
            if number < 20 {
                window.printw("The number is between 5 and 20\n");
            } else if number < 50 {
                window.printw("The number is between 20 and 50\n");
            } else if number < 65 {
                window.printw("The number is between 50 and 65\n");
            } else {
                window.printw("The number is between 65 and 80\n");
            }
        }
        _ => {
            if number < 90 {
                window.printw("The number is less than 90\n");
            } else {
                window.printw("The number is greater than 90\n");
            }
        }
    }
    window.refresh();
}
