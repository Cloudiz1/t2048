// use std::io::{self, Read};
//
// pub mod game;
//
// fn main() {
//     let mut game = game::Game::new();
//
//     game.set_board(game::DEBUG_BOARD);
//     game.print_board();
//
//     loop {
//         let mut buff = String::new();
//         _ = io::stdin().read_line(&mut buff);
//
//         match buff.trim() {
//             "a" => game.left(),
//             "d" => game.right(),
//             "s" => game.down(),
//             "w" => game.up(),
//             "r" => game.rotate_clockwise(),
//             _ => panic!("nuh uh"),
//         }
//
//         game.print_board();
//     }
// }

use std::io;

use crossterm::event::KeyModifiers;
use crossterm::ExecutableCommand;
use crossterm::{
    cursor::Hide,
    event::{read, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub mod game;

fn main() -> io::Result<()> {
    io::stdout().execute(Hide)?;
    enable_raw_mode()?;
    if let Err(e) = game_loop() {
        println!("Error: {e:?}\r");
    }
    disable_raw_mode()?;
    Ok(())
}

fn game_loop() -> io::Result<()> {
    let mut game = game::Game::new();
    game.spawn_block();
    game.spawn_block();
    // game.set_board(game::LOSS_BOARD);
    game.print_board();

    while let Ok(event) = read() {
        let Some(event) = event.as_key_press_event() else {
            continue;
        };

        let moved = match event.code {
            KeyCode::Char('w') | KeyCode::Up | KeyCode::Char('k') => {
                game.move_tiles(game.board, 'w', true)
            }
            KeyCode::Char('s') | KeyCode::Down | KeyCode::Char('j') => {
                game.move_tiles(game.board, 's', true)
            }
            KeyCode::Char('a') | KeyCode::Left | KeyCode::Char('h') => {
                game.move_tiles(game.board, 'a', true)
            }
            KeyCode::Char('d') | KeyCode::Right | KeyCode::Char('l') => {
                game.move_tiles(game.board, 'd', true)
            }
            _ => false,
        };

        if moved {
            game.spawn_block();
            game.print_board();
            match game.get_state() {
                game::State::Win => {
                    game.win();
                    break;
                }
                game::State::Loss => {
                    game.loss();
                    break;
                }
                game::State::MidGame => {}
            }
        }

        if event.code == KeyCode::Char('c') && event.modifiers.contains(KeyModifiers::CONTROL)
            || event.code == KeyCode::Esc
        {
            break;
        }
    }
    Ok(())
}

// fn main() {
//     let game = game::Game::new();
//
//     game.print_board(board);
// }
