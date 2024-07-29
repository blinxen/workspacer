use crossterm::cursor::{MoveTo, MoveToColumn, MoveToNextLine};
use crossterm::style::{Color, PrintStyledContent, SetForegroundColor, Stylize};
use crossterm::{cursor, QueueableCommand};
use std::io::stdout;
use std::io::Write;

// A Rect is a description of an area where:
// * x and y are the coordinates for the top left corner
// * width is the max value that x can get to
// * height is the max value that y can get to
#[derive(Clone)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub height: u16,
    pub width: u16,
}

// Draw border on a specific area
// Widget can use this function to draw borders
pub fn border(area: &Rect, title: &str) -> Result<(), std::io::Error> {
    let first_line = build_border_line('┌', '┐', '─', area.width);
    let last_line = build_border_line('└', '┘', '─', area.width);

    // Draw first line with nice curves
    stdout().queue(SetForegroundColor(Color::Yellow))?;
    stdout().queue(MoveTo(area.x, area.y))?;
    stdout().write_all(first_line.as_bytes())?;
    stdout().queue(cursor::MoveToColumn(
        area.x + ((area.width / 2) - title.len() as u16),
    ))?;
    stdout().queue(PrintStyledContent(title.reset().bold()))?;
    // We need to re-apply the background color since "PrintStyledContent" resets it
    stdout().queue(SetForegroundColor(Color::Yellow))?;
    // Draw vertical lines only on the left most and right most column
    for _ in 1..area.height {
        go_to_next_line_in_area(area, 0)?;
        stdout().write_all("│".as_bytes())?;
        stdout().queue(cursor::MoveRight(area.width - 2))?;
        stdout().write_all("│".as_bytes())?;
    }
    // Draw last line with nice curves
    go_to_next_line_in_area(area, 0)?;
    stdout().write_all(last_line.as_bytes())?;
    stdout().queue(SetForegroundColor(Color::Reset))?;

    Ok(())
}

// Helper method to help building borders easier
fn build_border_line(first: char, last: char, middle: char, length: u16) -> String {
    let mut line = String::new();
    line.push(first);
    for _ in 0..length - 2 {
        line.push(middle);
    }
    line.push(last);

    line
}

// Helper method to build a line and fill it with whitespace if it does not fill the whole row
pub fn build_line(content: &str, line_length: usize) -> String {
    let mut line = String::new();

    if line.len() >= line_length {
        content[..line_length].clone_into(&mut line);
    } else {
        line.push_str(content);
        line.push_str(&" ".repeat(line_length - content.len()));
    }

    line
}

// Go to the next line in a specific area
// x_offset can be set to allow moving on the X axis after the new line has been inserted
pub fn go_to_next_line_in_area(area: &Rect, x_offset: u16) -> Result<(), std::io::Error> {
    stdout().queue(MoveToNextLine(1))?;
    stdout().queue(MoveToColumn(area.x + x_offset))?;

    Ok(())
}

// Place cursor at the top left corner of an area
pub fn reset_cursor_in_area(area: &Rect) -> Result<(), std::io::Error> {
    stdout().queue(MoveTo(area.x, area.y))?;
    Ok(())
}
