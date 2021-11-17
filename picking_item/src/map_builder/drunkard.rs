use crate::prelude::*;
use super::MapArchitect;

//START: const_stagger
const STAGGER_DISTANCE: usize = 400;
//END: const_stagger
//START: coverage
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR : usize = NUM_TILES / 3;
//END: coverage

pub struct DrunkardsWalkArchitect {}

//START: build
impl MapArchitect for DrunkardsWalkArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder{
            map : Map::new(),
            rooms : Vec::new(),
            monster_spawns : Vec::new(),
            player_start : Point::zero(),
            amulet_start : Point::zero(),
            theme: super::themes::DungeonTheme::new()
        };

        mb.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH /2, SCREEN_HEIGHT/2);
        self.drunkard(&center, rng, &mut mb.map);
        while mb.map.tiles.iter()
            .filter(|t| **t == TileType::Floor).count() < DESIRED_FLOOR// <callout id="co.dwa.whileincomplete" />
        {
            self.drunkard(
                &Point::new(
                    rng.range(0, SCREEN_WIDTH),
                    rng.range(0, SCREEN_HEIGHT)
                ),
                rng,
                &mut mb.map
            );// <callout id="co.dwa.randomstart" />
            let dijkstra_map = DijkstraMap::new(// <callout id="co.dwa.dijkstra_build" />
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &vec![mb.map.point2d_to_index(center)],
                &mb.map,
                1024.0
            );
            dijkstra_map.map// <callout id="co.dwa.dijkstra_iter" />
                .iter()
                .enumerate()// <callout id="co.dwa.dijkstra_enumerate" />
                .filter(|(_, distance)| *distance > &2000.0)
                .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);
        }
        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}
//END: build

impl DrunkardsWalkArchitect {
    //START: drunkard
    fn drunkard(
        &mut self, 
        start: &Point, 
        rng: &mut RandomNumberGenerator,
        map: &mut Map
    ) {
        let mut drunkard_pos = start.clone();// <callout id="co.dwa.dposclone" />
        let mut distance_staggered = 0;// <callout id="co.dwa.zerostagger" />

        loop {// <callout id="co.dwa.loop" />
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;// <callout id="co.dwa.setfloor" />

            match rng.range(0, 4) {// <callout id="co.dwa.randomwalk" />
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1,
            }
            if !map.in_bounds(drunkard_pos) {// <callout id="co.dwa.exitmap" />
                break;
            }
 
            distance_staggered += 1;// <callout id="co.dwa.staggercheck" />
            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
    //END: drunkard
}