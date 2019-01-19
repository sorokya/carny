use tcod::RootConsole;

pub trait State {
    fn update(&mut self, console: &mut RootConsole);
    fn draw(&self, console: &mut RootConsole);
}
