mod app;
mod config;
mod terminal;
mod utils;
mod workspaces;

use app::App;
use crossterm::event::{self, Event, KeyEvent};

fn main() -> Result<(), std::io::Error> {
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
