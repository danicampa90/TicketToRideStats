use crate::board::Board;
use core::fmt::{Debug, Formatter};

pub struct GameState<'a> {
    built_routes: Vec<bool>,
    routes_to_ignore: Vec<bool>,
    board: &'a Board,
}

impl<'a> GameState<'a> {
    pub fn new(board: &'a Board) -> GameState<'a> {
        GameState {
            built_routes: [false].repeat(board.nr_routes()),
            routes_to_ignore: vec![],
            board: board,
        }
    }
    pub fn new_state_with_route(&'a self, route: usize) -> GameState<'a> {
        let mut new_built_routes = self.built_routes.clone();
        new_built_routes[route] = true;
        let mut new_routes_to_ignore = self.built_routes.clone();
        new_routes_to_ignore[route] = true;
        return GameState {
            built_routes: new_built_routes,
            routes_to_ignore: new_routes_to_ignore,
            board: self.board,
        };
    }
}

impl<'a> Debug for GameState<'a> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            formatter,
            "GameState (Built routes: {:?})",
            self.built_routes
        );
        Ok(())
    }
}
