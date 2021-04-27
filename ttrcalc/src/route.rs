#[derive(Debug)]
pub struct Route {
    pub city1: usize,
    pub city2: usize,
    pub tracks: Vec<Vec<TrackType>>,
    pub tracks_len: usize,
}

impl Route {
    pub fn new(city1: usize, city2: usize, tracks: Vec<Vec<TrackType>>) -> Route {
        // swap them to keep a consistent order
        let (city1, city2) = if city1 < city2 {
            (city1, city2)
        } else {
            (city2, city1)
        };
        Route {
            city1: city1,
            city2: city2,
            tracks_len: tracks[0].len(),
            tracks: tracks,
        }
    }
}

pub fn unknown_track_of_length(len: usize) -> Vec<Vec<TrackType>> {
    vec![[TrackType::Unknown].repeat(len)]
}

#[derive(Copy, Clone, Debug)]
pub enum TrackType {
    Normal(TrackColor),
    Tunnel(TrackColor),
    Locomotive,
    Unknown,
}

#[derive(Copy, Clone, Debug)]
pub enum TrackColor {
    Green,
    Blue,
    Red,
    Pink,
    Yellow,
    Anything,
}
