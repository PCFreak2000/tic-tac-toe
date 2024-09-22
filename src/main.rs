use std::io::{self, Write};

const SIZE: usize = 3;
type Board = [[FieldState; SIZE]; SIZE];

fn main() {
    let mut board: Board = [[FieldState::Empty; SIZE]; SIZE];

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print_board(board);

        println!("====================================================");

        print!("X's turn: ");
        let _ = stdout.flush();
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        let position: usize = buffer.trim_end().parse().expect("Only digits are allowed");
        player_turn(&mut board, position, FieldState::Cross);
        print_board(board);
        let winner = did_someone_win(&board);
        if winner != FieldState::Empty {
            println!("Winner is {:?}", winner);
            break;
        }

        println!("====================================================");

        print!("O's turn: ");
        let _ = stdout.flush();
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        let position: usize = buffer.trim_end().parse().expect("Only digits are allowed");
        player_turn(&mut board, position, FieldState::Circle);
        print_board(board);
        let winner = did_someone_win(&board);
        if winner != FieldState::Empty {
            println!("Winner is {:?}", winner);
            break;
        }

        println!("====================================================");
    }
}

fn did_someone_win(board: &Board) -> FieldState {
    // check the rows
    for row in board {
        if row.iter().all(|&e| e == FieldState::Cross) {
            return FieldState::Cross;
        }
        else if row.iter().all(|&e| e == FieldState::Circle) {
            return FieldState::Circle;
        }
    }

    // check the columns
    for column in 0..board.len() {
        if board[0][column] == FieldState::Cross
        && board[1][column] == FieldState::Cross
        && board[2][column] == FieldState::Cross {
            return FieldState::Cross;
        }
        else if board[0][column] == FieldState::Circle
             && board[1][column] == FieldState::Circle
             && board[2][column] == FieldState::Circle {
            return FieldState::Circle;
        }
    }

    // check the diagonals
    if board[0][0] == FieldState::Cross
    && board[1][1] == FieldState::Cross
    && board[2][2] == FieldState::Cross {
        return FieldState::Cross;
    }
    else if board[0][2] == FieldState::Cross
         && board[1][1] == FieldState::Cross
         && board[2][0] == FieldState::Cross {
        return FieldState::Cross;
    }
    else if board[0][0] == FieldState::Circle
         && board[1][1] == FieldState::Circle
         && board[2][2] == FieldState::Circle {
        return FieldState::Circle;
    }
    else if board[0][2] == FieldState::Circle
         && board[1][1] == FieldState::Circle
         && board[2][0] == FieldState::Circle {
        return FieldState::Circle;
    }

    FieldState::Empty
}

fn player_turn(board: &mut Board, position: usize, player: FieldState) {
    let position = position - 1;
    let row = position / 3;
    let column = position % 3;

    board[row][column] = player;
}

fn print_board(board: Board) {
    for (i, row) in board.iter().enumerate() {
        for (j, x) in row.iter().enumerate() {
            let symbol = match x {
                FieldState::Empty => ' ',
                FieldState::Cross => 'X',
                FieldState::Circle => 'O',
            };
        
            if j == row.len() - 1 {
                println!("{}", symbol);
            }
            else {
                print!("{} | ", symbol);
            }
        }

        if i != row.len() - 1 {
            println!("__|___|__");
            println!("  |   |  ");
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum FieldState {
    Empty,
    Cross,
    Circle
}
