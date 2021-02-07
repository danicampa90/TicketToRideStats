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

    let mut scheduler = task_system::Scheduler::new(2);
    scheduler.run(&|scheduler: &task_system::Scheduler, w: task_system::Work| vec![w])
}
