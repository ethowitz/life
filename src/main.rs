use std::{env, thread, time};
extern crate rand;

const ALIVE: char = 'x';

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

// i resent this function
fn get_live_neighbors(board: &Vec<Vec<char>>, row: usize, col: usize) -> u8 {
    let mut count: u8 = 0;
    for i in 0..3 {
        for j in 0..3 {
            if (i != 1 || j != 1) && (row + i) as i32 - 1 >= 0 && (col + j) as i32 - 1 >= 0 &&
               row + i - 1 < board.len() && col + j - 1 < board[0].len() &&
               board[row + i - 1][col + j - 1] == ALIVE {
                count += 1;
            }
        }
    }
    count
}

fn apply_rules(board: &Vec<Vec<char>>, rows: usize, cols: usize) -> Vec<Vec<char>> {
    let mut new_board = vec![vec![' '; cols]; rows];
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            let count = get_live_neighbors(board, i, j);
            if board[i][j] == ALIVE && (count < 2 || count > 3) {
                new_board[i][j] = ' ';
            } else if board[i][j] == ' ' && count == 3 {
                new_board[i][j] = ALIVE;
            } else {
                new_board[i][j] = board[i][j];
            }
        }
    }
    new_board
}

fn generate_seed(board: &mut Vec<Vec<char>>) {
    let gen_char = |x| if rand::random::<i64>() % 4 == 1 {ALIVE} else {' '};
    for row in board {
        *row = row.into_iter().map(&gen_char).collect();
    }
}

fn print_board(board: &Vec<Vec<char>>) {
    print!("\u{001b}c");
    for row in board {
        for square in row {
            print!("{}", square);
        }
        println!();
    }
}
