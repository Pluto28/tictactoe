// Simple tictactoe

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
    fn read_nick(msg: &str) -> String {
        let mut name = String::new();

        print!("{}", msg);
        stdout().flush().unwrap();
        // @TODO: This will panic in case there was an error reading the user's nickname.
        // Maybe printing an error message and trying again would be better
        stdin().lock().read_line(&mut name).unwrap();

        // remove newline
        name.pop();

        name
    }

    ///     Create a new game. Note that the Self is the same as the type the
    /// impl block is for.
    fn new() -> Self {
        let x_name = GameState::read_nick("Please insert the nick for the user playing X: ");
        let xplayer = Player::new(x_name, 'X');

        let o_name = GameState::read_nick("Please insert the nick for the user playing O: ");
        let oplayer = Player::new(o_name, 'O');

        // starting game state
        let gamestate = GameState {
            table: vec![' '; 9],
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
    fn row_separator(length: u32) {
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

            // So much validation to do. Users, amiright?
            //
            // since we popped the newline character, this string should now have
            // a length of 2(note that the len method gives us the size in bytes,
            // not the amount of characters). We also check if all characters
            // provided by the user are ascii, in which case each character has
            // one byte in size
            if input_buffer.len() == 2 && input_buffer.is_ascii() {
                // Make sure the row and column values are valid, throwing 
                // an error that asksa the user for new input if not
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
                    println!("\n\nColumn and/or row value too big(need to be");
                    println!("smaller than 3. Try again.\n\n");
                    continue;
                } else if row < 1 || column < 1 {
                    println!("\n\nColumn and/or row value too small(needs to be");
                    println!("bigger than 1). Try again.\n\n");
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

        (column, row)
    }

    fn is_updatable(&self, row: u8, col: u8) -> bool {
        let val = self.table.get(((row * 3) + col) as usize).unwrap();

        if *val == ' ' {
            true
        } else {
            false
        }
    }

    fn update_table(&mut self, row: u8, column: u8, playeri: usize) {
        let mark = self.players.get(playeri).unwrap().get_mark();

        let val = self.table.get_mut(((row * 3) + column) as usize).unwrap();
        *val = mark
    }

    // TODO: it's probably possible to replace the 3 functions that follow with
    // just one function which takes the starting row and column positions on the
    // table, the change to be applied to each value after each step, and iterate
    // through the table rows and columns, checking if all their characters match and
    // are not spaces, which are the characters being used to fill the starting array
    // (Maybe the array should be filled with the \0, or null, character)
    fn check_win_rows(&self) -> bool {
        let mut is_win: bool = true;
        let mut row = 0;

        while row != 3 {
            is_win = true;

            let mut start_mark = self.table.get(row * 3).unwrap();
            let table_slice = self.table.get((row * 3) + 1..(row + 1) * 3).unwrap();
            for mark in table_slice.iter() {
                if *start_mark == ' ' || *mark != *start_mark {
                    is_win = false;
                    break;
                }
                start_mark = mark;
            }

            if is_win {
                break;
            }

            row = row + 1;
        }

        is_win
    }

    fn check_win_columns(&self) -> bool {
        let mut is_win: bool = true;
        let mut row = 1;
        let mut col = 0;

        while col != 3 {
            is_win = true;

            let mut start_mark = self.table.get(col).unwrap();
            while row != 3 {
                let mark = self.table.get((row * 3) + col).unwrap();
                if *start_mark == ' ' || *mark != *start_mark {
                    is_win = false;
                    break;
                }

                start_mark = mark;
                row = row + 1;
            }

            if is_win {
                break;
            }

            col = col + 1;
        }

        is_win
    }

    fn check_win_diagonal2(&self) -> bool {
        let mut is_win = true;
        let mut row = 0;
        let mut column = 2;

        let mut start_mark = self.table.get((row * 3) + column).unwrap();
        row = row + 1;
        column = column - 1;

        while column != 0 {
            let mark = self.table.get((row * 3) + column).unwrap();
            //column = column - 1;

            if *start_mark == ' ' || *start_mark != *mark {
                is_win = false;
                break;
            }

            start_mark = mark;
            row = row + 1;
            column = column - 1;
        }

        is_win
    }
    fn check_win_diagonal1(&self) -> bool {
        let mut is_win = true;
        let mut row = 0;

        let mut start_mark = self.table.get((row * 3) + row).unwrap();
        row = row + 1;
        while row != 3 {
            let mark = self.table.get((row * 3) + row).unwrap();

            if *start_mark == ' ' || *start_mark != *mark {
                is_win = false;
                break;
            }
            start_mark = mark;
            row = row + 1;
        }

        is_win
    }

    fn check_win_diagonals(&self) -> bool {
        self.check_win_diagonal1() || self.check_win_diagonal2()
    }

    fn is_win(&self) -> bool {
        self.check_win_rows() || self.check_win_columns() || self.check_win_diagonals()
    }

    fn game_loop(&mut self) {
        let mut playeri = 0;
        loop {
            self.print_table();

            let name = self.players.get(playeri).unwrap().get_name();
            let (col, row) = Self::get_user_input(name);

            if self.is_updatable((row - 1) as u8, (col - 1) as u8) {
                self.update_table((row - 1) as u8, (col - 1) as u8, playeri);
            } else {
                println!("\n\nPlace is already taken\n\n");
                continue;
            }

            // the winner is the player that made the play this round, and so
            // the winer's index for the array of players is the actual player
            // index
            if self.is_win() {
                self.print_table();

                let player = self.players.get(playeri).unwrap();
                println!(
                    "\n\nCongratulations {}({}), you won!!!\n\n",
                    player.get_name(),
                    player.get_mark()
                );

                // TODO: Ask wheter the user wants to play another game
                break;
            }

            playeri = (playeri + 1) % 2;
        }
    }
}

fn main() {
    let mut state = GameState::new();
    state.game_loop();
}
