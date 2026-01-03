use crate::game;

// use std::collections::HashMap;
use std::vec::Vec;

static ACTIONS: [game::Direction; 4] = [
    game::Direction::Up,
    game::Direction::Down,
    game::Direction::Left,
    game::Direction::Right,
];

#[derive(Clone)]
struct Node {
    score: f64,
    state: [[i32; 4]; 4],
    best: game::Direction,
    // up: Box<Chance>,
    // left: Box<Chance>,
    // down: Box<Chance>,
    // right: Box<Chance>,
}

#[derive(Clone)]
struct Chance {
    score: f64,
    child_nodes: Vec<Box<(Node, f64)>>,
}

pub struct Bot {
    // cache: HashMap<[[i32; 4]; 4], (i32, i32, Vec<game::Direction>)>,
    game: game::Game,
}

impl Bot {
    pub fn new() -> Self {
        Self {
            // cache: HashMap::new(),
            game: game::Game::new(),
        }
    }

    fn make_moves(&mut self, state: [[i32; 4]; 4]) -> Vec<(bool, [[i32; 4]; 4], game::Direction)> {
        ACTIONS
            .iter()
            .map(|x| {
                let (moved, board) = self.game.move_tiles(state, x.clone(), false);
                return (moved, board, x.clone());
            })
            .collect()
    }

    fn score(&mut self, state: [[i32; 4]; 4]) -> f64 {
        if self.game.get_state() == game::State::Loss {
            return -9999.0;
        }

        let score_matrix = [
            [2048, 1024, 512, 256],
            [16, 32, 64, 128],
            [8, 4, 2, 2],
            [-8, -4, -2, 0],
        ];

        let mut score = 0.0;

        for (x, i) in std::iter::zip(state.iter(), score_matrix.iter()) {
            for (y, j) in std::iter::zip(x.iter(), i.iter()) {
                score += (*y as f64) * (*j as f64);
            }
        }

        let mut empty = 0.0;
        for row in state {
            for tile in row {
                if tile == 0 {
                    empty += 1.0;
                }
            }
        }

        score += empty * 12.0;
        return score;
    }

    fn generate_chance(&mut self, state: [[i32; 4]; 4], depth: i32) -> Chance {
        let mut states: Vec<Box<(Node, f64)>> = Vec::new();
        if depth > 0 {
            for x in 0..state.len() {
                for y in 0..state.len() {
                    if state[x][y] == 0 {
                        let mut tmp = state;
                        tmp[x][y] = 2;
                        states.push(Box::new((self.generate_tree(tmp, depth), 0.9)));

                        tmp[x][y] = 4;
                        states.push(Box::new((self.generate_tree(tmp, depth), 0.1)));
                    }
                }
            }
        }

        let mut score = 0.0;
        if states.len() == 0 {
            score = self.score(state);
        } else {
            for state in states.clone() {
                let (n, p) = *state;
                score += n.score * p;
            }
        }

        Chance {
            score,
            child_nodes: states,
        }
    }

    fn generate_tree(&mut self, state: [[i32; 4]; 4], depth: i32) -> Node {
        let mut children: Vec<(Chance, game::Direction)> = Vec::new();
        for (moved, state, direction) in self.make_moves(state) {
            if moved {
                let c = self.generate_chance(state, depth - 1);
                children.push((c, direction));
            }
        }

        let mut score = f64::NEG_INFINITY;
        let mut direction = game::Direction::Up;
        for (child, d) in children {
            if child.score > score {
                score = child.score;
                direction = d;
            }
        }

        Node {
            score,
            state,
            best: direction,
            // up: Box::new(children[0].clone()),
            // left: Box::new(children[1].clone()),
            // down: Box::new(children[2].clone()),
            // right: Box::new(children[3].clone()),
        }
    }

    fn get_move(&mut self, depth: i32) -> game::Direction {
        let tree = self.generate_tree(self.game.board, depth);
        return tree.best;
    }

    pub fn play(&mut self, depth: i32, delay: u64) {
        let time = std::time::Duration::from_millis(delay);
        loop {
            std::thread::sleep(time);
            let direction = self.get_move(depth);
            if !self.game.tick(direction) {
                break;
            }
        }
    }
}

/*
 * given a state:
 *   generate all possible other states
 *   recurse
 *   when you reach a desired depth:
 *      check lookup (only trust the score if it was calculated at an equal or higher depth)
 *      calculate score
 *      return
 *   hash:
 *      (state: (depth, score, Vec<Moves>))
*/
