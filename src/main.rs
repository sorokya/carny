extern crate tcod;

use tcod::{Console, RootConsole, Color, BackgroundFlag};
use tcod::input::Key;
use tcod::input::KeyCode::Escape;
pub mod consts;
mod states;
use states::state::State;
use states::menu::Menu;

fn main() {
    let mut root = RootConsole::initializer()
        .size(consts::GAME_WIDTH, consts::GAME_HEIGHT)
        .title("Carny")
        .init();

    let menu = Menu::new();
    
    while !root.window_closed() {
        root.set_default_foreground(tcod::colors::WHITE);
        root.set_default_background(tcod::colors::BLACK);

        root.clear();
        
        menu.draw(&mut root);
        
        root.flush();

        let key = root.wait_for_keypress(true);
        if key.pressed {
            match key {
                Key { code: Escape, .. } => break,
                _ => {}
            }
        }
    }
}
