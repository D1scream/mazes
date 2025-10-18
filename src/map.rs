use std::io::{self, BufRead};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Wall,
    Empty,
    Start,
    End,
    Path,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

pub struct Map {
    pub grid: Vec<Vec<Cell>>,
    pub start: Position,
    pub end: Position,
    pub rows: usize,
    pub cols: usize,
}

impl Map {
    pub fn parse_from_stdin() -> Result<Self, String> { 
        let stdin = io::stdin();
        let lines: Vec<String> = stdin.lock().lines()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Error reading input: {}", e))?;

        if lines.is_empty() {
            return Err("Empty input".to_string());
        }

        let mut map = Map {
            grid: Vec::new(),
            start: Position { row: 0, col: 0 },
            end: Position { row: 0, col: 0 },
            rows: 0,
            cols: 0,
        };
        let mut start_found = false;
        let mut end_found = false;

        for (row, line) in lines.iter().enumerate() {
            let mut row_cells = Vec::new();
            for (col, ch) in line.chars().enumerate() {
                let cell = match ch {
                    '#' => Cell::Wall,
                    ' ' => Cell::Empty,
                    'i' => {
                        if start_found {
                            return Err("Multiple start positions found".to_string());
                        }
                        start_found = true;
                        map.start = Position { row, col };
                        Cell::Start
                    }
                    'O' => {
                        if end_found {
                            return Err("Multiple end positions found".to_string());
                        }
                        end_found = true;
                        map.end = Position { row, col };
                        Cell::End
                    }
                    _ => return Err(format!("Invalid character: {}", ch)),
                };
                row_cells.push(cell);
            }
            map.grid.push(row_cells);
        }

        if !start_found {
            return Err("Start position 'i' not found".to_string());
        }
        if !end_found {
            return Err("End position 'O' not found".to_string());
        }

        map.rows = map.grid.len();
        map.cols = map.grid[0].len();

        Ok(map)
    }

    pub fn get_neighbors(&self, pos: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        
        for (dr, dc) in directions.iter() {
            let new_row = (pos.row as i32 + dr).rem_euclid(self.rows as i32) as usize;
            let new_col = (pos.col as i32 + dc).rem_euclid(self.cols as i32) as usize;
            
            let new_pos = Position { row: new_row, col: new_col };
            
            if self.grid[new_pos.row][new_pos.col] != Cell::Wall {
                neighbors.push(new_pos);
            }
        }
        
        neighbors
    }

    pub fn mark_path(&mut self, path: &[Position]) {
        for &pos in path {
            if self.grid[pos.row][pos.col] == Cell::Empty {
                self.grid[pos.row][pos.col] = Cell::Path;
            }
        }
    }

}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                let ch = match cell {
                    Cell::Wall => '#',
                    Cell::Empty => ' ',
                    Cell::Start => 'i',
                    Cell::End => 'O',
                    Cell::Path => '.',
                };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
