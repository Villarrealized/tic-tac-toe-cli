use std::io::{stdout, Write};
use std::process;

#[path = "./board.rs"]
mod board;
#[path = "./constants.rs"]
mod constants;
#[path = "./input.rs"]
mod input;
#[path = "./screen.rs"]
mod screen;
use board::Board;
use board::BoardLayout;
use board::BoardState;

/// Show game title and info
///
/// Waits for input before starting game
pub fn show_menu() {
    screen::clear();
    println!("{}", constants::GAME_START_INFO);

    print!("{}", constants::GAME_START_PROMPT);
    stdout().flush().unwrap();

    // wait for any input to continue
    let _ = input::get_input();

    start_game();
}

/// Draw the game board and enter main game loop
fn start_game() {
    let mut board = Board::new(BoardLayout::Numbered);
    board.draw();

    while let BoardState::Playing = board.state {
        // Get input from player
        let choice = prompt(&format!("{}'s turn: ", board.current_player));

        // Check if player wants to change the board layout
        if choice == "l" || choice == "toggle_layout" {
            board.toggle_layout();
            continue;
        } else if choice == "c" || choice == "toggle_color" {
            board.toggle_color();
            continue;
        }

        // Mark the board with player's choice
        if !board.mark(&choice) {
            println!("{}", constants::INVALID_MOVE_INFO);
            continue;
        }

        // Render
        board.draw();

        // Check for win/draw condition
        match board.check_winner() {
            Some(player) => {
                println!("");
                println!("{} wins!", player);
            }
            None => {
                if let BoardState::Draw = board.state {
                    println!("");
                    println!("Nobody wins. Cat's game!");
                } else {
                    board.next_turn()
                }
            }
        }
    }

    check_play_again();
}

/// Prompt for player input
///
/// Handles quitting & restarting game commands before passing result to caller
fn prompt(message: &str) -> String {
    print!("{}", message);
    // flush to make sure output is printed
    // not necessary if using println
    stdout().flush().unwrap();

    let input = input::get_input().unwrap();
    return match input.to_ascii_lowercase().as_str() {
        // if the input is empty, repeat until we get a value
        "" => prompt(message),
        "q" | "quit" | "exit" => {
            quit();
            String::from("")
        }
        "r" | "reset" | "restart" => {
            start_game();
            String::from("")
        }

        _ => input,
    };
}

/// Restart the game or quit
fn check_play_again() {
    let answer = prompt(&format!("{}", constants::PLAY_AGAIN_PROMPT));
    match answer.to_ascii_lowercase().as_str() {
        "y" | "yes" => {
            start_game();
        }
        "n" | "no" => {
            quit();
        }
        _ => check_play_again(),
    };
}

/// Quit the game and return to terminal
fn quit() {
    process::exit(0);
}
