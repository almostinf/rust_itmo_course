mod game;
use game::*;
use std::io;

fn match_comp_turn(game : &Game, res : WinOrNot) -> bool {
    if game.is_equal() {
        println!("Equal!!!");
        game.print_fields();
        return true
    }
    match res {
        WinOrNot::CompWin => {
            println!("Computer win!!!");
            game.print_fields();
            true
        }
        WinOrNot::None | WinOrNot::PlayerWin | WinOrNot::Equal => false
    }
}

fn main() {
    println!("Welcome to the tic-tac-toe!");
    println!("Enter X or 0(zero)");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read");
    let guess = guess.trim().as_bytes();
    if guess.len() != 1 {
        panic!("Error input!!!");
    }
    let comp_turn = if guess[0] == b'X' {
        Turns::Zero
    } else if guess[0] == b'0' {
        Turns::X
    } else {
        panic!("Error input!!!");
    };
    let mut game = Game::new(comp_turn);
    loop {
        println!("Game: ");
        game.print_fields();
        if comp_turn == Turns::X {
            let res = game.computer_turn();
            if res == WinOrNot::Equal {
                println!("Equal?");
            }
            if match_comp_turn(&game, res) {
                break;
            }
            game.print_fields();
        }
        println!("Enter you turn: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");
        let guess = guess.trim().as_bytes();
        let (x, y) = ((guess[0] as char).to_digit(10).unwrap(), (guess[2] as char).to_digit(10).unwrap());
        if x as usize > SIZE  || y as usize > SIZE {
            panic!("Error input!!!");
        }
        match game.player_turn((x - 1) as usize, (y - 1) as usize) {
            WinOrNot::PlayerWin => {
                println!("You win!!!");
                game.print_fields();
                break;
            }
            WinOrNot::Equal => {
                println!("Equal!!!");
                game.print_fields();
                break;
            }
            WinOrNot::None | WinOrNot::CompWin => (),
        }
        if comp_turn == Turns::Zero {
            let res = game.computer_turn();
            if match_comp_turn(&game, res) {
                break;
            }
        }
    }
}
