mod game;
mod parser;
mod route;

use game::Game;
use parser::parse_routes;
use route::Route;

fn main() {
    let mut game = Game::new(20);
    parse_routes("london_tracks.csv", &mut game);

    let routes: Vec<String> = game
        .routes_from_city(game.city_id("baker").unwrap())
        .map(|route| game.fmt_route(route))
        .collect();
    println!("{:?}", routes);
}
