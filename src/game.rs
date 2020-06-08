use std::collections::HashMap;
use std::rc::Rc;

use rand::SeedableRng;
use rand_pcg::Pcg32;
use serde::Deserialize;

use crate::combat::DamageInfo;
use crate::map::{Level, MapInfo};
use crate::monster::MonsterInfo;
use crate::player::Player;
use crate::point::Point;
use crate::screen::Screen;
use crate::tile::Tile;

pub struct Game {
    pub settings: GameSettings,
    pub map_info: MapInfo,
    pub monster_info: HashMap<String, Rc<MonsterInfo>>,
    pub damage_info: HashMap<String, DamageInfo>,
    pub menu: Rc<dyn Screen>,
    pub help: Rc<dyn Screen>,
    pub player: Player,
    pub levels: Levels,
    pub map_rng: Pcg32,
}

impl Game {
    pub fn new(
        settings: GameSettings,
        map_info: MapInfo,
        monster_info: HashMap<String, Rc<MonsterInfo>>,
        damage_info: HashMap<String, DamageInfo>,
        menu: Rc<dyn Screen>,
        help: Rc<dyn Screen>,
        seed: u64,
    ) -> Game {
        #[cfg(target_arch = "wasm32")]
        pub use stdweb::console;

        Game {
            player: Player {
                tile: settings.player.tile,
                pos: Point(1, 1),
            },
            settings,
            map_info,
            monster_info,
            damage_info,
            menu,
            help,
            levels: Levels {
                level: 0,
                floors: Vec::new(),
                basement: Vec::new(),
            },
            map_rng: SeedableRng::seed_from_u64(seed),
        }
    }
}

pub struct Levels {
    level: i32,
    floors: Vec<Level>,
    basement: Vec<Level>,
}
impl Levels {
    pub fn add_top(&mut self, level: Level) {
        self.floors.push(level);
    }
    pub fn cur(&self) -> &Level {
        if self.level < 0 {
            &self.basement[(-self.level + 1) as usize]
        } else {
            &self.floors[self.level as usize]
        }
    }
    pub fn cur_mut(&mut self) -> &mut Level {
        if self.level < 0 {
            &mut self.basement[(-self.level + 1) as usize]
        } else {
            &mut self.floors[self.level as usize]
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameSettings {
    pub interface: InterfaceSettings,
    pub player: PlayerSettings,
    pub map: MapSettings,
}
#[derive(Debug, Deserialize, Clone)]
pub struct InterfaceSettings {
    pub width: u32,
    pub height: u32,
    pub font: FontSettings,
    pub key_delay: u32,
}
#[derive(Debug, Deserialize, Clone)]
pub struct FontSettings {
    pub font: String,
    pub width: u32,
    pub height: u32,
}
#[derive(Debug, Deserialize, Clone)]
pub struct PlayerSettings {
    pub fov: i32,
    pub tile: Tile,
}
#[derive(Debug, Deserialize, Clone)]
pub struct MapSettings {
    pub place_attempts: i32,
    pub num_monsters: u32,
    pub width: usize,
    pub height: usize,
}
