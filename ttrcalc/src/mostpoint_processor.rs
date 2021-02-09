use crate::board::Board;
use crate::gamestate::GameState;
use crate::task_system::WorkProcessor;

use std::cell::RefCell;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct MostPointWorkProcessor<'a> {
    board: &'a Board,
    id: usize,
    log: bool,
    maximum_points: Arc<RwLock<(u32, GameState<'a>)>>,
}

impl<'a> MostPointWorkProcessor<'a> {
    pub fn new(board: &'a Board) -> MostPointWorkProcessor<'a> {
        return MostPointWorkProcessor {
            board: board,
            id: 0,
            log: false,
            maximum_points: Arc::new(RwLock::new((0, GameState::new(board)))),
        };
    }
    fn score_tracks(&self, state: &GameState) -> u32 {
        let mut points = 0;
        for (route_id, is_built) in state.built_routes().iter().enumerate() {
            if *is_built {
                let len = self.board.route_from_id(route_id).tracks_len;
                points += match len {
                    1 => 1,
                    2 => 2,
                    3 => 4,
                    4 => 7,
                    5 => 10,
                    6 => 21,
                    _ => panic!("Invalid lenght"),
                }
            }
        }
        return points;
    }
    fn score_missions(&self, state: &GameState) -> u32 {
        0
    }
}

#[derive(Debug)]
pub enum Work<'a> {
    Start,
    Explore(GameState<'a>),
    EvaluateScore(GameState<'a>),
}

impl<'a> Work<'a> {
    pub fn explore(state: GameState<'a>) -> Work<'a> {
        return Work::Explore(state);
    }
}
impl<'a> WorkProcessor<Work<'a>> for MostPointWorkProcessor<'a> {
    fn process(self: &Self, w: Work<'a>) -> Vec<Work<'a>> {
        let mut tasks = vec![];
        if self.log {
            println!("{:?}", w);
        }
        match w {
            Work::Start => {
                let mut routes_to_ignore = vec![];
                for route in 0..self.board.nr_routes() {
                    if self.log {
                        println!("... Build {}", route);
                    }
                    let new_state = GameState::new(self.board)
                        .new_state_with_route_id(route, &routes_to_ignore);
                    if new_state.is_ok() {
                        tasks.push(Work::explore(new_state.unwrap()));
                    }
                    routes_to_ignore.push(route);
                }
            }
            Work::Explore(gs) => {
                let mut routes_to_ignore = vec![];
                let neighbors = gs.neighboring_routes();
                if neighbors.len() == 0 {
                    if self.log {
                        println!("... Needs scoring {:?}", gs);
                    }
                    tasks.push(Work::EvaluateScore(gs))
                } else {
                    for route in neighbors {
                        let new_state = gs.new_state_with_route_id(route, &routes_to_ignore);
                        if new_state.is_ok() {
                            if self.log {
                                println!("... Build {} and enqueue {:?}", route, new_state);
                            }
                            tasks.push(Work::explore(new_state.unwrap()));
                        }
                        routes_to_ignore.push(route);
                    }
                }
            }
            Work::EvaluateScore(gs) => {
                let total_score = self.score_tracks(&gs) + self.score_missions(&gs);
                if self.log {
                    println!("... Score {:?} = {} pts", gs, total_score);
                }
                let maximum = self.maximum_points.read().unwrap();
                if maximum.0 < total_score {
                    // drop reader lock
                    drop(maximum);

                    let mut max_writable = self.maximum_points.write().unwrap();
                    if max_writable.0 < total_score {
                        println!("Set new max to {} points {:?}", total_score, gs);
                        (*max_writable) = (total_score, gs);
                    }
                }
            }
        }
        return tasks;
    }

    fn set_id(&mut self, id: usize) {
        self.id = id
    }
    fn done(&self) {
        println!("Thread {} is done", self.id);
    }
    fn sleep(&self, oth: usize) {
        println!("Thread {} is sleeping", self.id);
    }
    fn resume(&self, oth: usize) {}
    /*
    fn set_id(&mut self, id: usize) {}
    fn done(&self) {}
    fn sleep(&self, oth: usize) {}
    fn resume(&self, oth: usize) {}
    */
}
