use crate::route::Route;

pub struct Board {
    nr_trains: u32,
    routes: Vec<Route>,
    route_from_cities: Vec<Vec<usize>>,
    city_names: Vec<String>,
}

impl Board {
    pub fn new(nr_trains: u32) -> Board {
        Board {
            nr_trains: nr_trains,
            routes: vec![],
            route_from_cities: vec![],
            city_names: vec![],
        }
    }

    pub fn nr_trains(&self) -> u32 {
        self.nr_trains
    }

    pub fn add_route(&mut self, route: Route) {
        let index = self.routes.len();
        self.route_from_cities[route.city1].push(index);
        self.route_from_cities[route.city2].push(index);
        self.routes.push(route);
    }
    pub fn routes(&self) -> &Vec<Route> {
        &self.routes
    }
    pub fn route_from_id(&self, id: usize) -> &Route {
        &self.routes[id]
    }
    pub fn routes_from_city(&self, city: usize) -> impl Iterator<Item = &Route> {
        self.route_from_cities[city]
            .iter()
            .map(move |index| &self.routes[*index])
    }
    pub fn route_ids_from_city(&self, city: usize) -> &Vec<usize> {
        &self.route_from_cities[city]
    }
    pub fn nr_routes(&self) -> usize {
        self.routes.len()
    }

    pub fn city_names(&self) -> &Vec<String> {
        &self.city_names
    }
    pub fn city_id(&self, name: &str) -> Option<usize> {
        for index in 0..self.city_names.len() {
            if self.city_names[index] == name {
                return Some(index);
            }
        }
        return None;
    }
    pub fn city_name(&self, id: usize) -> &String {
        &self.city_names[id]
    }
    pub fn add_get_city(&mut self, name: String) -> usize {
        return match self.city_id(name.as_str()) {
            Some(x) => x,
            None => {
                self.city_names.push(name);
                self.route_from_cities.push(vec![]);
                self.city_names.len() - 1
            }
        };
    }

    pub fn fmt_route(&self, route: &Route) -> String {
        return format!(
            "Route({} --> {}. Tracks: {:?})",
            self.city_name(route.city1),
            self.city_name(route.city2),
            route.tracks
        );
    }
}
