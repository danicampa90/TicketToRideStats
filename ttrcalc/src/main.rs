mod board;
mod gamestate;
mod parser;
mod route;
mod task_system;

use board::Board;
use crossbeam::deque::{Injector, Stealer, Worker}; // https://docs.rs/crossbeam/0.8.0/crossbeam/deque/index.html
use gamestate::GameState;
use parser::parse_routes;
use route::Route;
use std::iter;
use task_system::{Work, WorkProcessor};

#[derive(Clone)]
struct MyWorkProcessor {
    id: usize,
}
impl WorkProcessor for MyWorkProcessor {
    fn process(self: &Self, w: Work) -> Vec<Work> {
        match w {
            Work::PrintDebug(i) => {
                println!("[{}]: {}", self.id, i);
                return if i < 20 {
                    vec![Work::PrintDebug(i + 1), Work::PrintDebug(i + 2)]
                } else {
                    vec![]
                };
            }
        }
    }
    fn set_id(&mut self, id: usize) {
        self.id = id
    }
    fn done(&self) {
        println!("Thread {} is done", self.id);
    }
    fn sleep(&self, oth: usize) {
        println!(
            "Thread {} is going to sleep. There are {} other threads sleeping",
            self.id, oth
        );
    }
    fn resume(&self, oth: usize) {
        println!(
            "Thread {} is waking up. There are {} other threads sleeping.",
            self.id, oth
        );
    }
}

fn main() {
    let mut game = Board::new(20);
    parse_routes("london_tracks.csv", &mut game);

    let routes: Vec<String> = game
        .routes_from_city(game.city_id("baker").unwrap())
        .map(|route| game.fmt_route(route))
        .collect();
    println!("{:?}", routes);

    let gamestate = GameState::new(&game);
    let gamestate = gamestate.new_state_with_route(1);
    println!("GameState: {:?}", gamestate);

    let mut scheduler = task_system::Scheduler::new(16);
    scheduler.push_task(Work::PrintDebug(1));
    scheduler.run(&MyWorkProcessor { id: 0 });
    println!("Done");
}
