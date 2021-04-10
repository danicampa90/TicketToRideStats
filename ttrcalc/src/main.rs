mod board;
mod debug_processor;
mod gamestate;
mod mostpoint_processor;
mod parser;
mod route;
mod task_system;

use board::Board;
use debug_processor::{DebugWork, DebugWorkProcessor};
use gamestate::GameState;
use mostpoint_processor::{MostPointCheckpointer, MostPointWorkProcessor, Work};
use parser::parse_routes;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    let mut game = Board::new(20);
    parse_routes("london_tracks.csv", &mut game);
    //parse_routes("europe_tracks.csv", &mut game);

    let mut scheduler = task_system::Scheduler::new(16, MostPointCheckpointer {});
    scheduler.push_task(Work::Start);
    let processor = MostPointWorkProcessor::new(&game);
    scheduler.run(&processor);

    let (max_score, max_state) = processor.into_maximum();
    println!("Maximum points: {}", max_score);
    explain_state(&max_state);
}

fn explain_state<'a>(state: &GameState<'a>) {
    println!("Built routes:");
    let board = state.board();
    for routeid in state.built_routes_list() {
        let route = board.route_from_id(routeid);
        let city1_name = board.city_name(route.city1);
        let city2_name = board.city_name(route.city2);
        println!(" - {}--[{}]-->{}", city1_name, route.tracks_len, city2_name);
    }
}
