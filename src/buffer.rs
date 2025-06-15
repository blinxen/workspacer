use crate::utils::Rect;
use crossterm::{
    cursor::MoveTo,
    style::{ContentStyle, PrintStyledContent, StyledContent},
    QueueableCommand,
};
use std::io::{stdout, Stdout, Write};

pub struct Buffer {
    current: Vec<Vec<StyledContent<char>>>,
    previous: Vec<Vec<StyledContent<char>>>,
    stdout: Stdout,
    pub area: Rect,
}

impl Buffer {
    // Create a buffer that has only empty cells
    pub fn new(area: Rect) -> Self {
        Buffer {
            current: vec![vec![Cell::default(); area.width as usize]; area.height as usize],
            previous: vec![vec![Cell::default(); area.width as usize]; area.height as usize],
            stdout: stdout(),
            area,
        }
    }

    pub fn resize(&mut self, area: Rect) {
        if self.area == area {
            return;
        }
        self.area = area.clone();
        self.previous = vec![vec![Cell::default(); area.width as usize]; area.height as usize];
        self.current = vec![vec![Cell::default(); area.width as usize]; area.height as usize];
    }

    fn relative_cell_position(&self, x: u16, y: u16) -> (u16, u16) {
        let (x, y) = (x - self.area.x, y - self.area.y);

        if y >= self.area.height {
            panic!(
                "Y coordinate outside of useable area ({} > {})",
                y, self.area.height
            )
        }

        if x >= self.area.width {
            panic!(
                "X coordinate outside of useable area ({} > {})",
                x, self.area.width
            )
        }

        (x, y)
    }

    pub fn write_string(&mut self, x: u16, y: u16, content: StyledContent<String>) {
        let (x, y) = self.relative_cell_position(x, y);
        for (i, char) in content.content().chars().enumerate() {
            self.current[y as usize][x as usize + i] = StyledContent::new(*content.style(), char);
        }
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        for (row, rows) in self.current.iter().enumerate() {
            for (column, cell) in rows.iter().enumerate() {
                if &self.previous[row][column] != cell {
                    self.stdout.queue(MoveTo(
                        column as u16 + self.area.x,
                        row as u16 + self.area.y,
                    ))?;
                    self.stdout.queue(PrintStyledContent(*cell))?;
                }
            }
        }

        self.stdout.flush()?;

        Ok(())
    }
}
