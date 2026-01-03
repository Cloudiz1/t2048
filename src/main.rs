use std::io;

use crossterm::event::KeyModifiers;
use crossterm::ExecutableCommand;
use crossterm::{
    cursor::Hide,
    event::{read, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub mod bot;
pub mod game;

#[derive(Debug)]
enum Mode {
    Normal,
    Endless,
    Bot,
}

fn parse_args() -> (Mode, i32, u64) {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        return (Mode::Normal, 0, 0);
    }

    let mut depth = 2;
    let mut delay = 20;
    let mut mode = Mode::Endless;
    for arg in args.iter().skip(1) {
        if *arg.to_ascii_lowercase() == "bot".to_owned() {
            mode = Mode::Bot;
        } else {
            let a: Vec<&str> = arg.split("=").collect();

            if a.len() == 1 {
                panic!("expected arguement form [variable]=[value].");
            }

            if a[0] == "depth" {
                depth = a[1]
                    .parse::<i32>()
                    .expect("expected integer on rhs of arguement");
            } else if a[0] == "delay" {
                delay = a[1]
                    .parse::<u64>()
                    .expect("expected integer on rhs of arguement");
            } else {
                panic!("unexpected arguement");
            }
        }
    }

    (mode, depth, delay)
}

fn main() -> io::Result<()> {
    io::stdout().execute(Hide)?;

    let (mode, depth, delay) = parse_args();
    match mode {
        Mode::Normal => {
            enable_raw_mode()?;

            if let Err(e) = game_loop() {
                println!("Error: {e:?}\r");
            }

            disable_raw_mode()?;
        }
        Mode::Bot => {
            bot::Bot::new().play(depth, delay);
        }
        Mode::Endless => {
            unimplemented!("to be added!");
        }
    }
    Ok(())
}

fn game_loop() -> io::Result<()> {
    let mut game = game::Game::new();
    // game.set_board(game::WIN_BOARD);
    game.print_board();

    while let Ok(event) = read() {
        let Some(event) = event.as_key_press_event() else {
            continue;
        };

        let direction: Option<game::Direction> = match event.code {
            KeyCode::Char('w') | KeyCode::Up | KeyCode::Char('k') => Some(game::Direction::Up),
            KeyCode::Char('s') | KeyCode::Down | KeyCode::Char('j') => Some(game::Direction::Down),
            KeyCode::Char('a') | KeyCode::Left | KeyCode::Char('h') => Some(game::Direction::Left),
            KeyCode::Char('d') | KeyCode::Right | KeyCode::Char('l') => {
                Some(game::Direction::Right)
            }
            _ => None,
        };

        if let Some(d) = direction {
            if !game.tick(d) {
                break;
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
