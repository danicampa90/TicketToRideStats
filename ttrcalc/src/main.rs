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
use mostpoint_processor::{MostPointWorkProcessor, Work};
use parser::parse_routes;

fn main() {
    let mut game = Board::new(20);
    //parse_routes("debug_tracks.csv", &mut game);
    parse_routes("london_tracks.csv", &mut game);
    /*
    let routes: Vec<String> = game
        .routes_from_city(game.city_id("baker").unwrap())
        .map(|route| game.fmt_route(route))
        .collect();
    println!("{:?}", routes);

    let gamestate = GameState::new(&game);
    let gamestate = gamestate.new_state_with_route_id(1, &vec![]);
    println!("GameState: {:?}", gamestate);
    return;*/
    let mut scheduler = task_system::Scheduler::new(16);
    scheduler.push_task(Work::Start);
    let processor = MostPointWorkProcessor::new(&game);
    scheduler.run(&processor);
    /* Debug */
    /*
    let mut scheduler = task_system::Scheduler::new(16);
    scheduler.push_task(DebugWork::PrintDebug(1));
    scheduler.run(&DebugWorkProcessor::new());
    println!("Done");
    */
}
