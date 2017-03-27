use std::{env, thread, time};
extern crate rand;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let rows = argv[1].parse::<usize>().unwrap();
    let cols = argv[2].parse::<usize>().unwrap();

    let mut board = vec![vec![' '; cols]; rows];
    generate_seed(&mut board);

    let delay = time::Duration::from_millis(100);
    loop {
        print_board(&board);
        board = apply_rules(&board, rows, cols);
        thread::sleep(delay);
    }
}

// TODO: replace with map
fn get_live_neighbors(board: &Vec<Vec<char>>, row: usize, col: usize) -> u8 {
    let mut count: u8 = 0;
    for i in 0..2 {
        for j in 0..2 {
            if (i != 0 || j != 0) && row + i < board.len() && col + j < board[0].len() && board[row + i][col + j] == 'X' {
                count += 1;
            }
        }
    }

    if row > 0 {
        for j in 0..2 {
            if row - 1 < board.len() && col + j < board[0].len() && board[row - 1][col + j] == 'X' {
                count += 1;
            }
        }
    }

    if col > 0 {
        for i in 0..2 {
            if row + i < board.len() && col - 1 < board[0].len() && board[row + i][col - 1] == 'X' {
                count += 1;
            }
        }
    }

    if col > 0 && row > 0 && board[row - 1][col - 1] == 'X' {
        count += 1;
    }

    count
}

fn apply_rules(board: &Vec<Vec<char>>, rows: usize, cols: usize) -> Vec<Vec<char>> {
    let mut new_board = vec![vec![' '; cols]; rows];
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            let count = get_live_neighbors(board, i, j);
            if board[i][j] == 'X' && (count < 2 || count > 3) {
                new_board[i][j] = ' ';
            } else if board[i][j] == ' ' && count == 3 {
                new_board[i][j] = 'X';
            } else {
                new_board[i][j] = board[i][j];
            }
        }
    }
    new_board
}

// TODO: replace with map
fn generate_seed(board: &mut Vec<Vec<char>>) {
    for row in board {
        for square in row {
            // change this to function of board size
            if rand::random::<i64>() % 4 == 1 {
                *square = 'X';
            }
        }
    }
}

// TODO: replace with map
fn print_board(board: &Vec<Vec<char>>) {
    print!("\u{001b}c");
    for row in board {
        for square in row {
            print!("{}", square);
        }
        println!();
    }
}
