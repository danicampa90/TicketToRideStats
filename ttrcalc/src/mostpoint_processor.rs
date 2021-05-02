use crate::board::Board;
use crate::gamestate::GameState;
use crate::route::Route;
use crate::task_system::WorkProcessingResult;
use crate::task_system::{Checkpointer, WorkProcessor};
use crossbeam::atomic::AtomicCell;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

use std::cell::RefCell;
use std::sync::{Arc, RwLock};

pub struct MostPointCheckpointer<'a> {
    board: &'a Board,
}
impl<'a> MostPointCheckpointer<'a> {
    pub fn new(board: &'a Board) -> MostPointCheckpointer {
        return MostPointCheckpointer { board: board };
    }
}

trait WriteableArray {
    fn write_array<T: Write>(&self, writer: &mut T) -> std::io::Result<()>;
}

impl WriteableArray for Vec<bool> {
    fn write_array<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        let mut build_routes_idx = vec![];
        for idx in 0..self.len() {
            if self[idx] {
                build_routes_idx.push(idx);
            }
        }
        writer.write_u16::<BigEndian>(build_routes_idx.len() as u16)?;
        for item in build_routes_idx {
            writer.write_u16::<BigEndian>(item as u16)?;
        }
        return Ok(());
    }
}
impl WriteableArray for Vec<usize> {
    fn write_array<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_u16::<BigEndian>(self.len() as u16)?;
        for &item in self {
            writer.write_u16::<BigEndian>(item as u16)?;
        }
        return Ok(());
    }
}

impl WriteableArray for Vec<Route> {
    fn write_array<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_u16::<BigEndian>(self.len() as u16)?;
        for item in self {
            writer.write_u16::<BigEndian>(item.city1 as u16)?;
            writer.write_u16::<BigEndian>(item.city2 as u16)?;
            writer.write_u16::<BigEndian>(item.tracks_len as u16)?;
        }
        return Ok(());
    }
}

impl WriteableArray for Vec<String> {
    fn write_array<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_u16::<BigEndian>(self.len() as u16)?;
        for item in self {
            let bytes = item.clone().into_bytes();
            writer.write_u16::<BigEndian>(bytes.len() as u16)?;
            writer.write(&bytes)?;
        }
        return Ok(());
    }
}

fn write_gamestate<T: Write>(writer: &mut T, work: &GameState) -> std::io::Result<()> {
    writer.write_u32::<BigEndian>(work.remaining_trains())?;
    work.built_routes().write_array(writer)?;
    work.cities_reached().write_array(writer)?;
    work.routes_to_ignore().write_array(writer)?;
    return Ok(());
}

fn write_board<T: Write>(writer: &mut T, board: &Board) -> std::io::Result<()> {
    writer.write_u32::<BigEndian>(board.nr_trains())?;
    board.city_names().write_array(writer)?;
    board.routes().write_array(writer)?;
    Ok(())
}

impl<'a> Checkpointer<Work<'a>> for MostPointCheckpointer<'a> {
    fn checkpoint(&self, work: &Vec<Work<'a>>) {
        println!("Checkpointing: {} tasks queued.", work.len());
        let mut writer = BufWriter::new(File::create("checkpoint").unwrap());
        // magic : dac0
        // version: 01 (major) 00 (minor)
        writer.write_u32::<BigEndian>(0xdaca0100).unwrap();
        // board magic marker
        writer.write_u32::<BigEndian>(0x626F7264).unwrap();
        // board
        write_board(&mut writer, &self.board).unwrap();
        // work magic marker
        writer.write_u32::<BigEndian>(0x71756575).unwrap();
        // len of work items
        writer.write_u32::<BigEndian>(work.len() as u32).unwrap();
        // list of work items
        for workitem in work {
            let (header_byte, gamestate) = match workitem {
                Work::Start => (1, None),
                Work::Explore(s) => (2, Some(s)),
                Work::EvaluateScore(s) => (3, Some(s)),
            };

            // work item header/enum value
            writer.write_u8(header_byte).unwrap();
            // work item
            if let Some(gamestate) = gamestate {
                write_gamestate(&mut writer, gamestate).unwrap();
            }
        }
        println!("Checkpoint finished");
    }
}

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
