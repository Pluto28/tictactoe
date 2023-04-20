// Simple tictactoe

use std::any::type_name;
use std::io::{stdin, stdout, BufRead, Write};
use std::process::exit;

struct Player {
    mark: char,
    name: String,
}

struct GameState {
    table: Vec<char>,
    round: u32,

    // the first player plays X and the second player plays O
    players: Vec<Player>,
}

impl Player {
    fn new(name: String, mark: char) -> Self {
        Player { mark, name }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_mark(&self) -> char {
        self.mark
    }
}

// impl blocks are used to implement behavior for structures or other data types
impl GameState {
    fn get_nick(msg: &str) -> String {
        let mut name = String::new();

        print!("{}", msg);
        stdout().flush();
        // @TODO: This will panic in case there was an error reading a nick from the user.
        // Maybe printing an error message and trying again would be better
        stdin().lock().read_line(&mut name).unwrap();

        // remove newline
        name.pop();

        name
    }

    ///     Create a new game. Note that the Self is the same as the type the
    /// impl block is for.
    fn new() -> Self {
        let x_name = GameState::get_nick("Please insert the nick for the user playing X: ");
        let xplayer = Player::new(x_name, 'X');

        let o_name = GameState::get_nick("Please insert the nick for the user playing O: ");
        let oplayer = Player::new(o_name, 'O');

        // Variable in rust are immutable by default.
        let mut gamestate = GameState {
            table: vec!['X'; 9],
            round: 1,
            players: vec![xplayer, oplayer],
        };

        gamestate
    }

    // & means to immutably borrow a value. Note that in rust, you can't have more
    // than  one mutable borrow for a value. If you try to mutably borrow a value more
    // than once, the compiler is gonna stop you, or, if using something like refcell,
    // you will have a runtime error. Fun stuff.
    //
    // print a separator of length - characters.
    fn row_separator(mut length: u32) {
        if !(length == 0) {
            print!("-");
            Self::row_separator(length - 1)
        }
    }

    fn print_table(&self) {
        let mut row_size = 0;
        for (ind, cellv) in self.table.iter().enumerate() {
            print!("| {} ", cellv);
            row_size = row_size + 4;

            if ((ind + 1) % 3) == 0 {
                print!("|");
                row_size = row_size + 1;

                print!("\n");
                GameState::row_separator(row_size);
                print!("\n");

                row_size = 0;
            }
        }
    }

    fn get_user_input(player_nick: &str) -> (u32, u32) {
        let mut row: u32;
        let mut column: u32;
        loop {
            println!(
                "Hi {}. Please type the row and column (<row><column>)",
                player_nick
            );
            print!("for the next play, or q to quit: ");
            stdout().flush().unwrap();
            let mut input_buffer: String = String::new();

            // reads user input
            stdin().lock().read_line(&mut input_buffer).unwrap(); // just panicking for now

            // remove the newline character at the end of the string
            input_buffer.pop();

            // since we popped the newline character, this string should now have
            // a length of 2(note that the len method gives us the size in bytes,
            // not the amount of characters). We also check if all characters
            // provided by the user are ascii, in which case each character has
            // one byte in size
            if input_buffer.len() == 2 && input_buffer.is_ascii() {
                row = match input_buffer.chars().nth(0).unwrap().to_digit(10) {
                    Some(row) => row,
                    None => {
                        println!("\n\nNon-decimal row value. Try again!\n\n");
                        continue;
                    }
                };

                column = match input_buffer.chars().nth(1).unwrap().to_digit(10) {
                    Some(column) => column,
                    None => {
                        println!("\n\nNon-decimal column value. Try again!\n\n");
                        continue;
                    }
                };

                if row > 3 || column > 3 {
                    println!("\n\nColumn or/and row value too big. Try again.\n\n");
                    continue;
                } 

                break;
            } else if input_buffer == "q" {
                exit(0);
            } else {
                println!("\n\nInvalid input. Try again.\n\n");
                continue;
            }
        }

        (0, 0)
    }

    fn is_updatable(&self, row: u8, col: u8) -> bool {
        let val = self.table.get(((row*3) + col) as usize).unwrap();

        if *val == ' ' {
            true
        } else  {
            false
        }
    }

    fn update_table(&mut self, row: u8, column: u8, mark: char) {
        let val = self.table.get_mut((row + column) as usize).unwrap();
    }

    fn game_loop(&mut self) {
        let mut playeri = 0;
        loop {
            self.print_table();

            let player = self.players.get(playeri).unwrap();
            let name = player.get_name();
            let mark = player.get_mark();

            let (col, row) = Self::get_user_input(name);

            if !self.is_updatable(row as u8, col as u8) {
                println!("\n\nPlace is already taken\n\n");
                continue;
            }

            playeri = (playeri + 1) % 2;
        }
    }
}

fn main() {
    let mut state = GameState::new();

    state.game_loop();
}
