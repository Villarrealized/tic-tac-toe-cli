#[path = "./player.rs"]
pub mod player;
use player::Player;
#[path = "./screen.rs"]
mod screen;

pub enum BoardState {
    Win,
    Draw,
    Playing,
}

pub enum BoardLayout {
    Numbered,
    Blank,
}

impl BoardLayout {
    fn value(&self) -> [&'static str; 9] {
        match self {
            BoardLayout::Numbered => ["1", "2", "3", "4", "5", "6", "7", "8", "9"],
            BoardLayout::Blank => [" "; 9],
        }
    }
}

pub struct Board {
    pub grid: [&'static str; 9],
    pub current_player: Player,
    pub state: BoardState,
    pub colorize: bool,
    layout: BoardLayout,
    turn: u8,
    max_turns: u8,
}

impl Board {
    pub fn new(layout: BoardLayout) -> Board {
        Board {
            grid: layout.value(),
            layout,
            colorize: false,
            turn: 1,
            current_player: Player::X,
            state: BoardState::Playing,
            max_turns: 9,
        }
    }

    pub fn mark(&mut self, grid_number: &str) -> bool {
        // convert the input to a usize, if possible
        // otherwise return false
        let grid_number: usize = match grid_number.parse() {
            Ok(number) => number,
            Err(_) => return false,
        };

        let index = grid_number - 1;
        // make sure index is not out of bounds
        if index >= self.grid.len() {
            return false;
        }

        // update game board if move is valid
        match self.current_player {
            Player::X => {
                if self.cell_is_empty(index) {
                    self.grid[index] = "X"
                } else {
                    return false;
                }
            }
            Player::O => {
                if self.cell_is_empty(index) {
                    self.grid[index] = "O"
                } else {
                    return false;
                }
            }
        }
        true
    }

    pub fn next_turn(&mut self) {
        // switch player
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
        // advance to next turn
        self.turn += 1;
    }

    /// Check the board for a win condition;
    pub fn check_winner(&mut self) -> Option<&Player> {
        // it takes a minimum of 5 turns to win in tic-tac-toe
        if self.turn < 5 {
            return None;
        }

        // Win patterns
        let horizontal_top = self.match_pattern([0, 1, 2]);
        let horizontal_center = self.match_pattern([3, 4, 5]);
        let horizontal_bottom = self.match_pattern([6, 7, 8]);

        let vertical_left = self.match_pattern([0, 3, 6]);
        let vertical_center = self.match_pattern([1, 4, 7]);
        let vertical_right = self.match_pattern([2, 5, 8]);

        let main_diaganol = self.match_pattern([0, 4, 8]);
        let anti_diaganol = self.match_pattern([2, 4, 6]);

        if horizontal_top
            || horizontal_center
            || horizontal_bottom
            || vertical_left
            || vertical_center
            || vertical_right
            || main_diaganol
            || anti_diaganol
        {
            self.state = BoardState::Win;
            return Some(&self.current_player);
        };

        // If we have filled up the whole board
        // and haven't won yet, it's a draw game
        if self.turn == self.max_turns {
            self.state = BoardState::Draw;
        }

        None
    }

    pub fn toggle_layout(&mut self) {
        match self.layout {
            BoardLayout::Blank => {
                let layout = BoardLayout::Numbered.value();
                for (index, cell) in self.grid.iter_mut().enumerate() {
                    *cell = match cell {
                        &mut "X" => "X",
                        &mut "O" => "O",
                        &mut _ => layout[index],
                    }
                }
                self.layout = BoardLayout::Numbered;
            }
            BoardLayout::Numbered => {
                for cell in self.grid.iter_mut() {
                    *cell = match cell {
                        &mut "X" => "X",
                        &mut "O" => "O",
                        _ => " ",
                    }
                }
                self.layout = BoardLayout::Blank;
            }
        }
        self.draw();
    }

    pub fn toggle_color(&mut self) {
        self.colorize = !self.colorize;
        self.draw();
    }

    pub fn draw(&self) {
        screen::clear();
        println!("");
        println!("   |   |   ");
        println!(
            " {} | {} | {} ",
            self.colorize(self.grid[0]),
            self.colorize(self.grid[1]),
            self.colorize(self.grid[2])
        );
        println!("___|___|___");
        println!("   |   |   ");
        println!(
            " {} | {} | {} ",
            self.colorize(self.grid[3]),
            self.colorize(self.grid[4]),
            self.colorize(self.grid[5])
        );
        println!("---|---|---");
        println!(
            " {} | {} | {} ",
            self.colorize(self.grid[6]),
            self.colorize(self.grid[7]),
            self.colorize(self.grid[8])
        );
        println!("   |   |   ");
        println!("");
    }

    /// Check the current player's mark against the
    /// `line` array given. If the current player's mark
    /// exists in each cell
    fn match_pattern(&self, line: [usize; 3]) -> bool {
        // Get the current player as an X or O str
        let mark = format!("{}", self.current_player);
        let mark = mark.as_str();

        if self.grid[line[0]] == mark && self.grid[line[1]] == mark && self.grid[line[2]] == mark {
            return true;
        }
        false
    }

    /// Check if a cell is playable on the board
    fn cell_is_empty(&self, index: usize) -> bool {
        match self.grid[index] {
            "X" | "O" => false,
            _ => true,
        }
    }

    /// Return the colorized version of the text
    /// if we have color enabled in the settings
    fn colorize<'a>(&self, text: &'a str) -> &'a str {
        if self.colorize {
            if text == "X" {
                return "\x1B[38;5;160mX\x1B[0m";
            } else if text == "O" {
                return "\x1B[38;5;26mO\x1B[0m";
            }
        }
        text
    }
}
