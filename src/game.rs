use rand;
use std::collections::HashMap;

pub static EMPTY_BOARD: [[i32; 4]; 4] = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
pub static WIN_BOARD: [[i32; 4]; 4] = [
    [1024, 512, 256, 128],
    [8, 16, 32, 64],
    [4, 2, 2, 0],
    [0, 0, 0, 0],
];

// should always generate a valid move
pub static VMR_TEST: [[i32; 4]; 4] = [
    [8, 16, 8, 16],
    [16, 8, 16, 8],
    [8, 16, 8, 16],
    [16, 8, 4, 0],
];

// should always generate a valid move
pub static LOSS_BOARD: [[i32; 4]; 4] = [
    [128, 64, 128, 64],
    [64, 128, 64, 128],
    [256, 1024, 256, 1024],
    [64, 128, 0, 0],
];

pub static DEBUG_BOARD: [[i32; 4]; 4] = [[0, 2, 2, 4], [2, 2, 2, 2], [0, 0, 0, 0], [0, 0, 0, 0]];

pub enum State {
    Win,
    Loss,
    MidGame,
}

pub struct Game {
    pub board: [[i32; 4]; 4],
    corner_padding: i32,
    colors: Colors,
    score: i32,
    moves: i32,
    // state: State,
}

struct Colors {
    backgrounds: HashMap<i32, String>,
    default: String,
    black_fg: String,
}

impl Game {
    pub fn new() -> Self {
        let mut backgrounds: HashMap<i32, String> = HashMap::new();
        backgrounds.insert(-1, "\x1B[48;2;194;180;169m".to_owned()); // background
        backgrounds.insert(0, "\x1B[48;2;208;193;185m".to_owned()); // empty cell
        backgrounds.insert(2, "\x1B[48;2;245;235;225m".to_owned());
        backgrounds.insert(4, "\x1B[48;2;236;224;200m".to_owned());
        backgrounds.insert(8, "\x1B[48;2;246;148;99m".to_owned());
        backgrounds.insert(16, "\x1B[48;2;245;148;103m".to_owned());
        backgrounds.insert(32, "\x1B[48;2;245;124;93m".to_owned());
        backgrounds.insert(64, "\x1B[48;2;244;95;53m".to_owned());
        backgrounds.insert(128, "\x1B[48;2;240;205;113m".to_owned());
        backgrounds.insert(256, "\x1B[48;2;238;202;105m".to_owned());
        backgrounds.insert(512, "\x1B[48;2;237;200;80m".to_owned());
        backgrounds.insert(1024, "\x1B[48;2;237;197;63m".to_owned());
        backgrounds.insert(2048, "\x1B[48;2;237;194;46m".to_owned());

        let colors = Colors {
            backgrounds,
            default: "\x1B[0m".to_owned(),
            black_fg: "\x1B[38;2;0;0;0m".to_owned(),
        };

        Game {
            corner_padding: 4,
            colors,
            board: EMPTY_BOARD,
            score: 0,
            moves: 0,
            // state: State::MidGame,
        }
    }

    pub fn set_board(&mut self, board: [[i32; 4]; 4]) {
        self.board = board;
    }

    pub fn get_left_padding(&self) -> String {
        let mut out: String = "".to_owned();
        out += &self.colors.default;
        for _ in 0..self.corner_padding {
            out += " ";
        }

        return out;
    }

    pub fn get_line_padding(&self) -> String {
        let mut line = "".to_owned();
        // each tile
        for _ in 0..4 {
            line += "         ";
        }

        // margin
        line += "  ";
        line += "\n\r";

        return line;
    }

    pub fn get_top_padding(&self) -> String {
        let mut lines = "".to_owned();

        for _ in 0..self.corner_padding / 2 {
            lines += &self.get_left_padding();
            lines += &self.get_line_padding();
        }

        return lines;
    }

    pub fn print_board(&self) {
        // clear board
        print!("{}", self.colors.default);
        print!("\x1B[2J\x1B[1;1H");

        let mut line = "".to_owned();
        line += &self.get_top_padding();

        line += "        ";
        line += "Score: ";
        line += &format!("{:<10}", self.score);

        line += "    ";
        line += "Moves: ";
        line += &format!("{:<6}", self.moves);
        line += "\n\r";

        line += &self.get_left_padding();
        line += self.colors.backgrounds.get(&-1).unwrap();
        line += &self.get_line_padding();

        for row in self.board {
            // each square
            for i in 0..3 {
                line += &self.get_left_padding();
                line += self.colors.backgrounds.get(&-1).unwrap();
                line += "  ";
                for col in row {
                    let color = self.colors.backgrounds.get(&col).unwrap();
                    line += color;

                    if i == 1 && col > 0 {
                        line += &self.colors.black_fg;
                        line += &format!("{:^7}", col);
                    } else {
                        line += "       ";
                    }

                    line += self.colors.backgrounds.get(&-1).unwrap();
                    line += "  ";
                    line += &self.colors.default;
                }

                line += "\n\r";
            }

            line += &self.get_left_padding();
            line += self.colors.backgrounds.get(&-1).unwrap();
            line += &self.get_line_padding();

            print!("{}", line);
            line = "".to_owned();
        }
    }
}

impl Game {
    pub fn spawn_block(&mut self) {
        let val = match rand::random_range(0..=10) {
            0..=9 => 2,
            10 => 4,
            _ => unreachable!(),
        };

        let mut empty_spaces: Vec<(usize, usize)> = Vec::new();
        for i in 0..self.board.len() {
            for j in 0..self.board.len() {
                if self.board[i][j] == 0 {
                    empty_spaces.push((i, j));
                }
            }
        }

        if empty_spaces.len() == 0 {
            return;
        }

        let index = rand::random_range(0..empty_spaces.len());
        let (i, j) = empty_spaces[index];
        self.board[i][j] = val;

        if empty_spaces.len() == 1 {
            if self.valid_state(self.board) {
                return;
            }

            let new_val = match val {
                2 => 4,
                4 => 2,
                _ => panic!("unexpected value"),
            };

            self.board[i][j] = new_val;
        }
    }

    fn valid_state(&mut self, state: [[i32; 4]; 4]) -> bool {
        let moves: [char; 4] = ['w', 'a', 's', 'd'];
        for m in moves {
            if self.move_tiles(state, m, false) {
                return true;
            }
        }

        return false;
    }

    fn transpose(&self, board: [[i32; 4]; 4]) -> [[i32; 4]; 4] {
        let mut out = board;
        for y in 0..out.len() {
            for x in (y + 1)..out.len() {
                let tmp = out[x][y];
                out[x][y] = out[y][x];
                out[y][x] = tmp;
            }
        }

        return out;
    }

    fn rotate_clockwise(&self, board: [[i32; 4]; 4]) -> [[i32; 4]; 4] {
        let mut out = self.transpose(board);
        for i in 0..out.len() {
            let mut tmp = out[i];
            tmp.reverse();
            out[i] = tmp;
        }

        return out;
    }

    pub fn move_tiles(
        &mut self,
        state: [[i32; 4]; 4],
        direction: char,
        change_state: bool,
    ) -> bool {
        let mut buffer = state;
        let score;
        let moved;
        match direction {
            'w' => {
                buffer = self.rotate_clockwise(buffer);
                buffer = self.rotate_clockwise(buffer);
                buffer = self.rotate_clockwise(buffer);
                (buffer, score, moved) = self.left(buffer);
                buffer = self.rotate_clockwise(buffer);
            }
            'a' => (buffer, score, moved) = self.left(buffer),
            's' => {
                buffer = self.rotate_clockwise(buffer);
                (buffer, score, moved) = self.left(buffer);
                buffer = self.rotate_clockwise(buffer);
                buffer = self.rotate_clockwise(buffer);
                buffer = self.rotate_clockwise(buffer);
            }
            'd' => {
                buffer = self.rotate_clockwise(buffer);
                buffer = self.rotate_clockwise(buffer);
                (buffer, score, moved) = self.left(buffer);
                buffer = self.rotate_clockwise(buffer);
                buffer = self.rotate_clockwise(buffer);
            }
            _ => panic!("invalid direction"),
        };

        if change_state {
            self.score += score;
            self.moves += moved as i32;
            self.board = buffer;
        }

        return moved;
    }

    pub fn left(&mut self, state: [[i32; 4]; 4]) -> ([[i32; 4]; 4], i32, bool) {
        let mut out = state;
        let mut matches: Vec<(usize, usize)> = Vec::new();
        let mut moved: bool = false;
        let mut score = 0;
        for i in 0..out.len() {
            'piece: for j in 1..out.len() {
                if out[i][j] == 0 {
                    continue;
                }

                let mut k: usize = j - 1;
                while out[i][k] == 0 && k > 0 {
                    k -= 1;
                }

                if k == 0 && out[i][0] == 0 {
                    out[i][0] = out[i][j];
                    out[i][j] = 0;
                    moved = true;
                    // match!
                } else if out[i][k] == out[i][j] {
                    // prevents double merges
                    for m in matches.clone() {
                        if (i, k) == m {
                            out[i][k + 1] = out[i][j];
                            out[i][j] = 0;
                            continue 'piece;
                        }
                    }

                    matches.push((i, k));

                    out[i][k] *= 2;
                    out[i][j] = 0;
                    score += out[i][k];
                    moved = true;
                } else if k + 1 != j {
                    out[i][k + 1] = out[i][j];
                    out[i][j] = 0;
                    moved = true;
                }
            }
        }

        if moved {
            (out, score, moved)
        } else {
            (out, 0, moved)
        }
    }

    pub fn get_state(&mut self) -> State {
        if !self.valid_state(self.board) {
            return State::Loss;
        }

        for row in self.board {
            for val in row {
                if val == 2048 {
                    return State::Win;
                }
            }
        }

        return State::MidGame;
    }

    pub fn win(&self) {
        print!("{}", self.colors.default);
        print!("{}", self.get_left_padding());
        println!("     You win! Final score: {}\r", self.score);
        println!("\r");
    }

    pub fn loss(&self) {
        print!("{}", self.colors.default);
        print!("{}", self.get_left_padding());
        println!("     Game over! Final score: {}\r", self.score);
        println!("\r");
    }
}
