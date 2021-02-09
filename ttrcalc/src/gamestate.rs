use crate::board::Board;
use core::fmt::{Debug, Formatter};

pub struct GameState<'a> {
    built_routes: Vec<bool>,
    routes_to_ignore: Vec<bool>,
    cities_reached: Vec<usize>,
    board: &'a Board,
    remaining_trains: u32,
}

impl<'a> GameState<'a> {
    pub fn new(board: &'a Board) -> GameState<'a> {
        GameState {
            built_routes: [false].repeat(board.nr_routes()),
            routes_to_ignore: [false].repeat(board.nr_routes()),
            board: board,
            cities_reached: vec![],
            remaining_trains: board.nr_trains(),
        }
    }

    pub fn built_routes(&self) -> &Vec<bool> {
        &self.built_routes
    }
    pub fn new_state_with_route_id(
        &self,
        route: usize,
        additional_to_ignore: &Vec<usize>,
    ) -> Result<GameState<'a>, ()> {
        let route_len = self.board.route_from_id(route).tracks_len as u32;
        if self.remaining_trains < route_len {
            return Err(());
        }

        let mut new_built_routes = self.built_routes.clone();
        new_built_routes[route] = true;
        let mut new_routes_to_ignore = self.routes_to_ignore.clone();
        new_routes_to_ignore[route] = true;
        for toignore in additional_to_ignore {
            new_routes_to_ignore[*toignore] = true;
        }
        let route_info = &self.board.routes()[route];

        let mut new_cities_reached = self.cities_reached.clone();
        if let Err(idx) = self.cities_reached.binary_search(&route_info.city1) {
            new_cities_reached.insert(idx, route_info.city1);
        }
        if let Err(idx) = self.cities_reached.binary_search(&route_info.city2) {
            new_cities_reached.insert(idx, route_info.city2);
        }

        return Ok(GameState {
            built_routes: new_built_routes,
            routes_to_ignore: new_routes_to_ignore,
            board: self.board,
            cities_reached: new_cities_reached,
            remaining_trains: self.remaining_trains - route_len,
        });
    }

    pub fn neighboring_routes(&self) -> Vec<usize> {
        let mut neighbor_routes_bitfield = Vec::with_capacity(self.board.nr_routes());
        neighbor_routes_bitfield.resize_with(self.board.nr_routes(), || false);
        for city in &self.cities_reached {
            for route in self.board.route_ids_from_city(*city) {
                neighbor_routes_bitfield[*route] = true;
            }
        }
        let mut result = vec![];
        for (pos, is_neighbor) in neighbor_routes_bitfield.iter().enumerate() {
            if *is_neighbor && !self.routes_to_ignore[pos] {
                result.push(pos);
            }
        }
        return result;
    }
}

impl<'a> Debug for GameState<'a> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut built = vec![];
        for (pos, isbuilt) in self.built_routes.iter().enumerate() {
            if *isbuilt {
                built.push(pos);
            }
        }
        let mut ignored = vec![];
        for (pos, isignored) in self.routes_to_ignore.iter().enumerate() {
            if *isignored {
                ignored.push(pos);
            }
        }
        write!(
            formatter,
            "GameState (Built: {:?}, Ignored: {:?})",
            built, ignored
        );
        Ok(())
    }
}
