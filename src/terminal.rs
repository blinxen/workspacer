use crate::utils::Rect;
use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crossterm::{terminal, QueueableCommand};
use std::io::{self, Error};

// Enable some terminal features that will be required for this app to work
pub fn prepare_terminal() -> Result<(), Error> {
    // Disable some default Terminal features that are not required
    enable_raw_mode()?;
    // Enable mouse support and make sure terminal starts in an alternate screen
    // Alternate means that the current screen is restored after exiting the application
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    io::stdout().queue(cursor::Hide)?;

    Ok(())
}

// Cleanup work that needs to be done after exiting the application
// This can include reseting the screen, re-enabling some terminal features etc.
pub fn restore_terminal() -> Result<(), Error> {
    disable_raw_mode()?;

    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    execute!(io::stdout(), cursor::Show)?;

    Ok(())
}

// Return the terminal size as a Rect
pub fn size() -> Result<Rect, Error> {
    let (x, y) = terminal::size()?;

    Ok(Rect {
        x: 0,
        y: 0,
        width: x,
        height: y,
    })
}
