use crate::board::{HEIGHT, WIDTH, Board};
use rand::{seq::IteratorRandom, rngs::ThreadRng};
use tch::Tensor;


pub struct Env {
    board: Board,
    thread_rng: ThreadRng
}

pub struct Step<A> {
    pub obs: Tensor,
    pub action: A,
    pub reward: f64,
    pub is_done: bool,
}

impl Env {
    pub fn new() -> Env {
        Env { 
            board: Board::new(),
            thread_rng: rand::thread_rng()
        }   
    }

    pub fn reset(&mut self) -> Tensor {
        self.board = Board::new();
        self.play_random_move();
        self.to_tensor()
    }

    pub fn step(&mut self, action: i64) -> Step<i64> {
        let mut is_done = false;
        let mut reward = 0.0;
        let placement = self.board.place(action as usize, 2);
        if placement.is_ok() {
            reward -= 1.0;
            if self.board.finished() {
                is_done = true;
            } else {
                self.play_random_move();
            }
        } else {
            reward -= 5.0;
        }

        if let Some(winner) = self.board.winner() {
            if winner == 1 {
                reward -= 100.0;
            } else if winner == 2 {
                reward += 100.0;
            }
        }
        
        Step { obs: self.to_tensor(), action, reward: reward, is_done }
    }

    pub fn action_space(&self) -> i64 {
        WIDTH as i64
    }

    pub fn observation_space(&self) -> Vec<i64> {
        (0..(WIDTH * HEIGHT)).map(|i| i as i64).collect::<Vec<i64>>()
    }

    fn play_random_move(&mut self) {
        let cols = self.board.available_columns();
        let _ = self.board.place(cols.iter().choose(&mut self.thread_rng).unwrap().to_owned(), 1);
    }

    fn to_tensor(&self) -> Tensor {
        Tensor::from_slice(&self.board.flatten()).to_kind(tch::Kind::Float)
    }
}

impl Step<i64> {
    pub fn copy_with_obs(&self, obs: &Tensor) -> Step<i64> {
        Step { obs: obs.copy(), action: self.action, reward: self.reward, is_done: self.is_done }
    }
}
