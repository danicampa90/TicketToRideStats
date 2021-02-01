mod board;
mod gamestate;
mod parser;
mod route;

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
}

fn find_task<T>(local: &Worker<T>, global: &Injector<T>, stealers: &[Stealer<T>]) -> Option<T> {
    // Pop a task from the local queue, if not empty.
    local.pop().or_else(|| {
        // Otherwise, we need to look for a task elsewhere.
        iter::repeat_with(|| {
            // Try stealing a batch of tasks from the global queue.
            global
                .steal_batch_and_pop(local)
                // Or try stealing a task from one of the other threads.
                .or_else(|| stealers.iter().map(|s| s.steal()).collect())
        })
        // Loop while no task was stolen and any steal operation needs to be retried.
        .find(|s| !s.is_retry())
        // Extract the stolen task, if there is one.
        .and_then(|s| s.success())
    })
}
