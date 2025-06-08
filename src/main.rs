mod app;
mod buffer;
mod config;
mod terminal;
mod utils;
mod workspaces;

use app::App;
use crossterm::event::{self, Event, KeyEvent};

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        terminal::restore_terminal().expect("Could no restore terminal");
        hook(info);
    }));
}

fn main() -> Result<(), std::io::Error> {
    set_panic_hook();
    terminal::prepare_terminal()?;
    let mut app = App::new();
    loop {
        if app.quit {
            break;
        }

        app.render()?;
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            app.handle_key_event(code);
        }
    }
    terminal::restore_terminal()?;

    Ok(())
}
