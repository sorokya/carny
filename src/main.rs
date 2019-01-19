extern crate tcod;

use tcod::{Console, RootConsole, Color, BackgroundFlag};
use tcod::input::Key;
use tcod::input::KeyCode::Escape;
mod states;
use states::menu::Menu;

fn main() {
    const WIDTH: i32 = 120;
    const HEIGHT: i32 = 75;
    const FPS: i32 = 20;

    let mut root = RootConsole::initializer()
        .size(WIDTH, HEIGHT)
        .title("Carny")
        .init();

    let menu = Menu::new();


    while !root.window_closed() {
        root.set_default_foreground(tcod::colors::WHITE);
        root.set_default_background(tcod::colors::BLACK);

        root.clear();
        
        root.set_default_background(tcod::colors::WHITE);
        root.set_default_foreground(tcod::colors::BLACK);
        //root.vertical_line(28, 4, 9, BackgroundFlag::Default);

        root.print_frame(0, 0, 30, HEIGHT, false, BackgroundFlag::Set, Some(""));
        
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
