use crate::prelude::*;
use super::MapArchitect;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
    //START: build
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder{
            map : Map::new(),
            rooms: Vec::new(),
            monster_spawns : Vec::new(),
            player_start : Point::zero(),
            amulet_start : Point::zero(),
            theme: super::themes::DungeonTheme::new()
        };
        self.random_noise_map(rng, &mut mb.map);
        for _ in 0..10 {
            self.iteration(&mut mb.map);
        }
        let start = self.find_start(&mb.map);
        mb.monster_spawns = mb.spawn_monsters(&start, rng);
        mb.player_start = start;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
    //END: build
}

impl CellularAutomataArchitect {
    //START: random_noise
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|t| {// <callout id="co.ca.each" />
            let roll = rng.range(0, 100);// <callout id="co.ca.roll100" />
            if roll > 55 {// <callout id="co.ca.bias" />
                *t = TileType::Floor;// <callout id="co.ca.floor" />
            } else {
                *t = TileType::Wall;
            }
        });
    }
    //END: random_noise

    //START: count_neighbors
    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;
        for iy in -1 ..= 1 {
            for ix in -1 ..= 1 {
                if !(ix==0 && iy == 0) &&
                    map.tiles[map_idx(x+ix, y+iy)] == TileType::Wall
                {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }
    //END: count_neighbors

    //START: iteration
    fn iteration(&mut self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();// <callout id="co.ca.iterclone" />
        for y in 1 .. SCREEN_HEIGHT -1 {// <callout id="co.ca.itermap" />
            for x in 1 .. SCREEN_WIDTH -1 {
                let neighbors = self.count_neighbors(x, y, map);// <callout id="co.ca.neighborcount" />
                let idx = map_idx(x, y);
                if neighbors > 4 || neighbors == 0 {// <callout id="co.ca.iterif" />
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
        map.tiles = new_tiles;
    }
    //END: iteration

    //START: find_start
    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);// <callout id="co.ca.findcenter" />
        let closest_point = map.tiles
            .iter()// <callout id="co.ca.iter_all" />
            .enumerate()// <callout id="co.ca.enumerate" />
            .filter(|(_, t)| **t == TileType::Floor)// <callout id="co.ca.filter_floor" />
            .map(|(idx, _)| (idx, DistanceAlg::Pythagoras.distance2d(// <callout id="co.ca.distance" />
                center,
                map.index_to_point2d(idx)
            )))
            .min_by(|(_, distance), (_, distance2)| distance.partial_cmp(&distance2).unwrap())// <callout id="co.ca.min_by" />
            .map(|(idx, _)| idx)// <callout id="co.ca.map" />
            .unwrap();// <callout id="co.ca.unwrap" />
        map.index_to_point2d(closest_point)// <callout id="co.ca.convert" />
    }
    //END: find_start
}