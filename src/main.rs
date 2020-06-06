use std::collections::hash_map::DefaultHasher;
use std::env;
use std::hash::Hasher;
use std::rc::Rc;

use doryen_rs::{App, AppOptions};

pub mod game;
use game::Game;

pub mod screen;
// use screen::game::GameScreen;
use screen::menu::MenuScreen;
use screen::textbox::TextBox;
use screen::{Action, WheatleyEngine};

pub mod colors;
pub mod map;
pub mod player;
pub mod point;
pub mod tile;

use colors::{BLACK, DARK_GREEN};
use tile::Tile;

const SCREEN_WIDTH: u32 = 100;
const SCREEN_HEIGHT: u32 = 45;

// const LIMIT_FPS: i32 = 30;

pub const PLAYER_TILE: Tile = Tile {
    ch: '@' as u16,
    fg: DARK_GREEN,
    bg: BLACK,
};
// pub const PLAYER_FOV: i32 = 10;

fn main() {
    let mut app = App::new(AppOptions {
        console_width: SCREEN_WIDTH,
        console_height: SCREEN_HEIGHT,
        screen_width: SCREEN_WIDTH * 8,
        screen_height: SCREEN_HEIGHT * 12,
        window_title: "Wheatley Simulator".to_owned(),
        font_path: "curses_vector_8x12.png".to_owned(),
        vsync: false,
        ..AppOptions::default()
    });

    //     tcod::system::set_fps(LIMIT_FPS);

    let mut hasher = DefaultHasher::new();

    env::args().nth(1).map(|s| hasher.write(s.as_bytes()));

    let seed: u64 = hasher.finish();

    let help = Rc::new(TextBox::new(
        Some(String::from("Help")),
        String::from(
            r#"? - Show help screen
Arrow keys: move around
You can also use numpad or vi-keys:
  7 8 9    y k u
  4 @ 6    h @ l
  1 2 3    b j n"#,
        ),
        50,
        30,
        true,
    ));

    let game = Game::new(Rc::new(MenuScreen::new(String::from(
r#"+-------------------------------------------------------------------------+
|           __          ___                _   _                          |
|           \ \        / / |              | | | |                         |
|            \ \  /\  / /| |__   ___  __ _| |_| | ___ _   _               |
|             \ \/  \/ / | '_ \ / _ \/ _` | __| |/ _ \ | | |              |
|              \  /\  /  | | | |  __/ (_| | |_| |  __/ |_| |              |
|               \/  \/   |_| |_|\___|\__,_|\__|_|\___|\__, |              |
|                                                      __/ |              |
|                                                     |___/               |
|   _____ _                 _       _               ___   ___ ___   ___   |
|  / ____(_)               | |     | |             |__ \ / _ \__ \ / _ \  |
| | (___  _ _ __ ___  _   _| | __ _| |_ ___  _ __     ) | | | | ) | | | | |
|  \___ \| | '_ ` _ \| | | | |/ _` | __/ _ \| '__|   / /| | | |/ /| | | | |
|  ____) | | | | | | | |_| | | (_| | || (_) | |     / /_| |_| / /_| |_| | |
| |_____/|_|_| |_| |_|\__,_|_|\__,_|\__\___/|_|    |____|\___/____|\___/  |
+-------------------------------------------------------------------------+"#),
            vec![
                // (String::from("Play!"), Action::Push(Rc::new(GameScreen))),
                (String::from("Help"), Action::Push(help.clone())),
                (String::from("Credits"), Action::Push(Rc::new(TextBox::new(
                    Some(String::from("Credits")),
                    String::from("Game by Paul Maynard\nFlavor text contributed by:\n - Joyce Quach\ncurses_vector tileset by DragonDePlatino"),
                    50, 20, true
                )))),
                (String::from("Quit"), Action::Pop),
            ]
        )), help, seed);

    let screens = WheatleyEngine::new(game);

    app.set_engine(Box::new(screens));

    app.run();
}
