use std::{ io::{self, Write}, usize};
use rand::Rng;

//Define the size of the game board
const BOARD_SIZE: usize = 10;

struct Board{
    grid: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    ships: Vec<(usize, usize)>,
}
#[derive(Clone, Copy, PartialEq)]
enum CellState{
    Empty,
    Ship,
    Hit,
    Miss,
}

impl Board {
    fn new() -> Self{
        Board{
            grid: [[CellState::Empty;BOARD_SIZE]; BOARD_SIZE],
            ships: Vec::new(),
        }
    }

    //Randomly place ships
    fn place_ship(&mut self, size:usize){
        let mut rng = rand::thread_rng();

        loop {
            let row = rng.gen_range(0..BOARD_SIZE);
            let col = rng.gen_range(0..BOARD_SIZE);
            let direction = rng.r#gen::<bool>();
            if self.can_place_ship(row, col, size,direction){
                for i in 0..size{
                    let (r,c) = if direction{
                        (row, col+i)
                    } else {
                        (row+i, col)
                    };
                    self.grid[r][c] = CellState::Ship;
                    self.ships.push((r,c))
                }
                break;
            }
        }
    }
    //Check if the ship can be placed without conflicts
    fn can_place_ship(&self, row:usize, col:usize, size: usize, direction:bool) -> bool {
        if direction{
            if col + size > BOARD_SIZE { 
                return false; 
            }
            for i in 0..size{
                if self.grid[row][col+i] != CellState::Empty {return false;}
            }
        } else {
            if row + size > BOARD_SIZE { return false;}
            for i in 0..size{
                if self.grid[row+i][col] != CellState::Empty {return false;}
            }
        }
        true
    }

    //Attack a cell
    fn attack(&mut self, row:usize, col:usize) -> CellState{
        match self.grid[row][col]{
            CellState::Empty =>{
                self.grid[row][col] = CellState::Miss;
                CellState::Miss
            },
            CellState::Ship => {
                self.grid[row][col] = CellState::Hit;
                CellState::Hit
            },
            _ => CellState::Miss,
        }
    }


    //display the board
    fn display(&self, hide_ships: bool) {
        print!("   ");
        for i in 0..BOARD_SIZE { print!(" {} ", i); }
        println!();
        for (i, row) in self.grid.iter().enumerate() {
            print!("{:2} ", i);
            for cell in row {
                match cell {
                    CellState::Empty => {
                        if hide_ships {
                            print!("   ");
                        } else {
                            print!(" \u{25A1} ");  //Water
                        }
                    }
                    CellState::Ship => {
                        if hide_ships { print!("   "); } else { print!(" \u{25A0} "); }  //Ship pieces
                    }
                    CellState::Hit => print!("\x1b[31m \u{25CF} \x1b[0m"),  //Hit (red)
                    CellState::Miss => print!("\x1b[36m \u{00B7} \x1b[0m"), //Miss (blue)
                }
            }
            println!();
        }
    }

    fn is_game_over(&self) -> bool {
        self.ships.iter().all(|&(r,c)| self.grid[r][c] == CellState::Hit)
    }
}


fn main() {

    let mut player_board = Board::new();
    let mut opponent_board = Board::new();


    player_board.place_ship(5); // Aircraft Carrier
    player_board.place_ship(4); // Battleship
    player_board.place_ship(3); // Cruiser
    player_board.place_ship(3); // Submarine
    player_board.place_ship(2); // Destroyer

    opponent_board.place_ship(5);
    opponent_board.place_ship(4);
    opponent_board.place_ship(3);
    opponent_board.place_ship(3);
    opponent_board.place_ship(2);

    // Main game loop
    loop {
        // Clear the screen
        print!("\x1b[2J\x1b[1;1H");

        println!("\x1b[1;37mYour Board:\x1b[0m");
        player_board.display(false);
        println!("\x1b[1;37mOpponent's Board:\x1b[0m");
        opponent_board.display(true);

        let (player_row, player_col) = get_player_input();
        let result = opponent_board.attack(player_row, player_col);
        match result {
            CellState::Miss => println!("\x1b[36mYou missed!\x1b[0m"),
            CellState::Hit => println!("\x1b[31mYou hit a ship!\x1b[0m"),
            _ => (),
        }
        println!("Press Enter to continue...");
        io::stdin().read_line(&mut String::new()).expect("Failed to read line");

        
        if opponent_board.is_game_over() {
            println!("\x1b[1;32mCongratulations! You sank all of your opponent's ships!\x1b[0m");
            break;
        }

        // Opponent's turn: simulate opponent move (could be AI-controlled in future enhancements)
        let (opponent_row, opponent_col) = generate_opponent_move();
        let result = player_board.attack(opponent_row, opponent_col);
        match result {
            CellState::Miss => println!("\x1b[36mOpponent missed!\x1b[0m"),
            CellState::Hit => println!("\x1b[31mOpponent hit one of your ships!\x1b[0m"),
            _ => (),
        }
        println!("Press Enter to continue...");
        io::stdin().read_line(&mut String::new()).expect("Failed to read line");

        if player_board.is_game_over() {
            println!("\x1b[1;31mOh no! All of your ships have been sunk!\x1b[0m");
            break;
        }
    }
}

fn get_player_input() -> (usize, usize) {
    loop{
        print!("\x1b[1;37mEnter coordinates to fire (row,col): \x1b[0m");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let coordinates: Vec<usize> = input.trim().split(',').map(|s| s.trim().parse().expect("Invalid Input")).collect();
        if coordinates.len() == 2 && coordinates[0] < BOARD_SIZE && coordinates[1] < BOARD_SIZE {
            return (coordinates[0], coordinates[1]);
        } else {
            println!("x1b[1;31mInvalid input. Please enter row and column numbers")
        }
    }
}

fn generate_opponent_move() -> (usize, usize) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..BOARD_SIZE), rng.gen_range(0..BOARD_SIZE))
}


