use crate::buffer::Buffer;
use crossterm::style::{ContentStyle, StyledContent, Stylize};

// A Rect is a description of an area where:
// * x and y are the coordinates for the top left corner
// * width is the max value that x can get to
// * height is the max value that y can get to
#[derive(Clone, PartialEq, Eq)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub height: u16,
    pub width: u16,
}

pub fn border(
    buffer: &mut Buffer,
    title: &str,
    header: Option<StyledContent<&str>>,
    footer: Option<StyledContent<&str>>,
) {
    let first_line = build_border_line('┌', '┐', '─', buffer.area.width);
    let last_line = build_border_line('└', '┘', '─', buffer.area.width);
    // Add offset if header or footer is defined to make everything fit
    let top_offset = if header.is_some() { 1 } else { 0 };
    let bottom_offset = if footer.is_some() { 1 } else { 0 };

    // Draw Header
    if let Some(header) = header {
        buffer.write_str(
            header.content(),
            buffer.area.x,
            buffer.area.y,
            *header.style(),
        );
    }

    // Draw first line with nice curves
    buffer.write_str(
        &first_line,
        buffer.area.x,
        buffer.area.y + top_offset,
        ContentStyle::default().yellow(),
    );
    buffer.write_str(
        title,
        buffer.area.x + ((buffer.area.width / 2) - title.len() as u16),
        buffer.area.y + top_offset,
        ContentStyle::default().bold(),
    );
    // Draw vertical lines only on the left most and right most column
    for i in 1..buffer.area.height - top_offset - bottom_offset {
        buffer.write_str(
            "│",
            buffer.area.x,
            buffer.area.y + top_offset + i,
            ContentStyle::default().yellow(),
        );
        buffer.write_str(
            "│",
            buffer.area.x + buffer.area.width - 1,
            buffer.area.y + top_offset + i,
            ContentStyle::default().yellow(),
        );
    }
    // Draw last line with nice curves
    buffer.write_str(
        &last_line,
        buffer.area.x,
        buffer.area.y + buffer.area.height - 1 - bottom_offset,
        ContentStyle::default().yellow(),
    );

    // Draw footer
    if let Some(footer) = footer {
        buffer.write_str(
            footer.content(),
            buffer.area.x,
            buffer.area.y + buffer.area.height - 1,
            *footer.style(),
        );
    }
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
pub fn build_line(content: String, line_length: usize) -> String {
    let mut line = String::new();

    if line.len() >= line_length {
        content[..line_length].clone_into(&mut line);
    } else {
        line.push_str(&content);
        line.push_str(&" ".repeat(line_length - content.len()));
    }

    line
}
