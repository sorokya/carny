use super::state::State;
use tcod::{Console, RootConsole, Color, BackgroundFlag};
use tcod::input::Key;
use super::super::consts;


pub struct Menu {
}

impl Menu {
    pub fn new() -> Menu {
        Menu { }
    }
}

impl State for Menu {
    fn update(&mut self, console: &mut RootConsole) {
        use tcod::input::Key;
        use tcod::input::KeyCode::*;

        let key = console.wait_for_keypress(true);
        if key.pressed {
            match key {
                _ => {}
            }
        }
    }
    
    fn draw(&self, console: &mut RootConsole) {
        console.set_default_background(tcod::colors::WHITE);
        console.set_default_foreground(tcod::colors::BLACK);
        console.print_frame(0, 0, 30, consts::GAME_HEIGHT, false, BackgroundFlag::Set, Some("test"));
    }
}
