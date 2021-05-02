use crate::task_system::WorkProcessingResult;
use crate::task_system::{Checkpointer, WorkProcessor};
use crossbeam::atomic::AtomicCell;
use ttrmodels::Board;
use ttrmodels::GameState;
use ttrmodels::Route;

use std::cell::RefCell;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct MostPointWorkProcessor<'a> {
    board: &'a Board,
    id: usize,
    log: bool,
    maximum_points: Arc<RwLock<(u32, GameState<'a>)>>,
    exploration_steps: Arc<AtomicCell<u64>>,
    scoring_steps: Arc<AtomicCell<u64>>,
}

impl<'a> MostPointWorkProcessor<'a> {
    pub fn new(board: &'a Board) -> MostPointWorkProcessor<'a> {
        return MostPointWorkProcessor {
            board: board,
            id: 0,
            log: false,
            maximum_points: Arc::new(RwLock::new((0, GameState::new(board)))),
            exploration_steps: Arc::new(AtomicCell::new(0)),
            scoring_steps: Arc::new(AtomicCell::new(0)),
        };
    }
    fn score_tracks(&self, state: &GameState) -> u32 {
        let mut points = 0;
        for route_id in state.built_routes_list() {
            let len = self.board.route_from_id(route_id).tracks_len;
            points += match len {
                1 => 1,
                2 => 2,
                3 => 4,
                4 => 7,
                6 => 15,
                8 => 21,
                _ => panic!("Invalid length in score_tracks"),
            }
        }
        return points;
    }
    fn score_missions(&self, state: &GameState) -> u32 {
        0
    }

    pub fn into_maximum(self) -> (u32, GameState<'a>) {
        println!(
            "Exploration steps: {}",
            self.exploration_steps.as_ref().load()
        );
        println!("Scoring steps: {}", self.scoring_steps.as_ref().load());
        let max = Arc::<RwLock<(u32, GameState<'a>)>>::try_unwrap(self.maximum_points).unwrap();
        return max.into_inner().unwrap();
        //return self.maximum_points.try_unwrap();
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
    fn process(self: &Self, w: Work<'a>) -> WorkProcessingResult<Work<'a>> {
        let mut tasks = vec![];
        if self.log {
            println!(">{:?}", w);
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
                self.exploration_steps.as_ref().fetch_add(1);
                let mut routes_to_ignore = vec![];
                let neighbors = gs.neighboring_routes();
                let mut has_explored = false;
                for route in neighbors {
                    let new_state = gs.new_state_with_route_id(route, &routes_to_ignore);
                    if self.log {
                        println!("... {} -> {:?}", route, new_state);
                    }
                    if new_state.is_ok() {
                        has_explored = true;
                        if self.log {
                            println!("... Build {} and enqueue {:?}", route, new_state);
                        }
                        tasks.push(Work::explore(new_state.unwrap()));
                    }
                    routes_to_ignore.push(route);
                }
                if !has_explored {
                    if self.log {
                        println!("... Needs scoring {:?}", gs);
                    }
                    tasks.push(Work::EvaluateScore(gs))
                }
            }
            Work::EvaluateScore(gs) => {
                let scoring_steps = self.scoring_steps.as_ref().fetch_add(1);
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
                if scoring_steps % 10000000 == 0 {
                    return WorkProcessingResult::AddWorkAndCheckpoint(tasks);
                }
            }
        }
        return WorkProcessingResult::AddWork(tasks);
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
    fn resume(&self, oth: usize) {
        println!("Thread {} is resuming", self.id);
    }
    /*
    fn set_id(&mut self, id: usize) {}
    fn done(&self) {}
    fn sleep(&self, oth: usize) {}
    fn resume(&self, oth: usize) {}
    */
}
