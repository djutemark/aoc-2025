#![allow(dead_code, unused)]

use std::{error::Error, ops::Add, usize};

const SAMPLE_INPUT: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

fn main() {
    const INPUT: &str = include_str!("input.txt");

    let mut  grid = Grid::parse(INPUT).unwrap();

    println!(
        "Total number of removable paper rolls: {}",
        grid.remove_until_impossible()
    );
}

fn is_accessible(num_paper_roll_neighbours: usize) -> bool {
    num_paper_roll_neighbours < 4
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Location {
    Empty,
    PaperRoll,
}

impl Location {
    const EMPTY_SYMBOL: char = '.';
    const PAPER_ROLL_SYMBOL: char = '@';

    fn get_symbol(&self) -> char {
        match self {
            Location::Empty => Self::EMPTY_SYMBOL,
            Location::PaperRoll => Self::PAPER_ROLL_SYMBOL,
        }
    }

    fn parse_symbol(symbol: char) -> Result<Self, &'static str> {
        match symbol {
            Self::EMPTY_SYMBOL => Ok(Location::Empty),
            Self::PAPER_ROLL_SYMBOL => Ok(Location::PaperRoll),
            _ => Err("Invalid symbol"),
        }
    }
}

enum GridDirection {
    Up,
    UpRight,
    Right,
    RightDown,
    Down,
    DownLeft,
    Left,
    LeftUp,
}

#[derive(PartialEq, Debug)]
struct GridPosition {
    x: i16,
    y: i16,
}

impl GridPosition {
    fn offset_from_direction(dir: &GridDirection) -> Self {
        match dir {
            GridDirection::Up => GridPosition { x: 0, y: -1 },
            GridDirection::UpRight => GridPosition { x: 1, y: -1 },
            GridDirection::Right => GridPosition { x: 1, y: 0 },
            GridDirection::RightDown => GridPosition { x: 1, y: 1 },
            GridDirection::Down => GridPosition { x: 0, y: 1 },
            GridDirection::DownLeft => GridPosition { x: -1, y: 1 },
            GridDirection::Left => GridPosition { x: -1, y: 0 },
            GridDirection::LeftUp => GridPosition { x: -1, y: -1 },
        }
    }
}

impl Add<&GridPosition> for &GridPosition {
    type Output = GridPosition;

    fn add(self, rhs: &GridPosition) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Grid {
    locations: Vec<Location>,
    width: usize,
}

impl Grid {
    fn new(width: usize) -> Self {
        Self {
            locations: vec![],
            width,
        }
    }

    fn total_locations(&self) -> usize {
        self.locations.len()
    }

    fn add_row(&mut self, row: GridRow) -> Result<(), &'static str> {
        let locations = row.locations();

        if locations.len() != self.width {
            return Err("Row to add was not of expected width");
        }

        for location in locations {
            self.locations.push(location);
        }

        Ok(())
    }

    fn parse(input: &str) -> Result<Self, &'static str> {
        let mut lines = input.lines().peekable();

        let next_line = lines.peek();
        if next_line == None {
            return Err("");
        }

        let next_line = next_line.unwrap();

        let width = next_line.len();

        let mut locations: Vec<Location> = Vec::new();

        for line in input.lines() {
            if line.len() != width {
                return Err("");
            }

            let row = GridRow::parse(line);

            for location in row.locations() {
                locations.push(location);
            }
        }

        Ok(Self { locations, width })
    }

    fn get_neighbour(&self, pos: &GridPosition, dir: &GridDirection) -> Option<Location> {
        let grid_offset = GridPosition::offset_from_direction(dir);

        let pos = &grid_offset + pos;

        if pos.x < 0 || (pos.x as usize) >= self.width {
            return None;
        }
        if pos.y < 0 || (pos.y as usize) >= self.height() {
            return None;
        }

        let index = (pos.x as usize) + (pos.y as usize) * self.width;

        if index >= self.locations.len() {
            return None;
        }

        Some(self.locations[index])
    }

    fn height(&self) -> usize {
        self.locations.len() / self.width
    }

    fn get_num_paper_roll_neighbours(&self, pos: &GridPosition) -> usize {
        let mut num_valids = 0;

        if let Some(n) = self.get_neighbour(&pos, &GridDirection::Up)
            && n == Location::PaperRoll
        {
            num_valids += 1;
        }
        if let Some(n) = self.get_neighbour(&pos, &GridDirection::UpRight)
            && n == Location::PaperRoll
        {
            num_valids += 1;
        }
        if let Some(n) = self.get_neighbour(&pos, &GridDirection::Right)
            && n == Location::PaperRoll
        {
            num_valids += 1;
        }
        if let Some(n) = self.get_neighbour(&pos, &GridDirection::RightDown)
            && n == Location::PaperRoll
        {
            num_valids += 1;
        }
        if let Some(n) = self.get_neighbour(&pos, &GridDirection::Down)
            && n == Location::PaperRoll
        {
            num_valids += 1;
        }
        if let Some(n) = self.get_neighbour(&pos, &GridDirection::DownLeft)
            && n == Location::PaperRoll
        {
            num_valids += 1;
        }
        if let Some(n) = self.get_neighbour(&pos, &GridDirection::Left)
            && n == Location::PaperRoll
        {
            num_valids += 1;
        }
        if let Some(n) = self.get_neighbour(&pos, &GridDirection::LeftUp)
            && n == Location::PaperRoll
        {
            num_valids += 1;
        }

        num_valids
    }

    fn position_from_index(&self, index: usize) -> GridPosition {
        let x = (index % self.width) as i16;
        let y = (index / self.height()) as i16;

        GridPosition { x, y }
    }

    fn index_from_position(&self, pos: &GridPosition) -> usize {
        (pos.x as usize) + (pos.y as usize) * self.width
    }

    fn is_index_accessible(&self, index: usize) -> bool {
        if self.locations[index] != Location::PaperRoll {
            return false;
        }

        let num_paper_roll_neighbours =
            self.get_num_paper_roll_neighbours(&self.position_from_index(index));
        is_accessible(num_paper_roll_neighbours)
    }

    fn is_position_accessible(&self, pos: &GridPosition) -> bool {
        let index = self.index_from_position(pos);

        self.is_index_accessible(index)
    }

    fn get_accessible_indices(&self) -> Vec<usize> {
        let mut v: Vec<usize> = Vec::new();

        for i in 0..self.total_locations() {
            if self.is_index_accessible(i) {
                v.push(i);
            }
        }

        v
    }

    fn get_num_accessible(&self) -> usize {
        self.get_accessible_indices().len()
    }

    fn render_to_string(&self) -> String {
        let mut s = String::new();

        for i in 0..self.locations.len() {
            if i != 0 && i % self.width == 0 {
                s.push_str("\n");
            }

            if self.is_index_accessible(i) {
                s.push_str("x");
            } else {
                s.push_str(&self.locations[i].get_symbol().to_string());
            }
        }

        s
    }

    fn remove_accessible(&mut self) -> NumRemoved {
        let removable_indices = self.get_accessible_indices();
        let num_removed = removable_indices.len();

        for i in removable_indices {
            self.locations[i] = Location::Empty;
        }

        num_removed
    }

    fn remove_until_impossible(&mut self) -> TotalRemoved {
        let mut total_removed = 0;
        
        while let num_removed = self.remove_accessible() && num_removed != 0 {
            total_removed += num_removed;
        }

        total_removed
    }
}

type NumRemoved = usize;
type TotalRemoved = usize;

struct GridRow(Vec<Location>);

impl GridRow {
    fn parse(input: &str) -> Self {
        let mut locations: Vec<Location> = Vec::new();

        for ch in input.chars() {
            let location = Location::parse_symbol(ch).unwrap();

            locations.push(location);
        }

        Self(locations)
    }

    fn locations(self) -> Vec<Location> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use std::i16;

    use crate::*;

    const SAMPLE_INPUT_ROW: &str = "..@@.@@@@.";

    const SAMPLE_INPUT_UPPER_FULL: &str = r"@@@
...
...";
    const SAMPLE_INPUT_UPPER_EMPTY: &str = r"...
@@@
@@@";

    #[test]
    fn test_get_location_symbol() {
        assert_eq!(Location::Empty.get_symbol(), '.');
        assert_eq!(Location::PaperRoll.get_symbol(), '@');
    }

    #[test]
    fn test_parse_location_symbol() {
        assert_eq!(Location::parse_symbol('.'), Ok(Location::Empty));
        assert_eq!(Location::parse_symbol('@'), Ok(Location::PaperRoll));

        assert_eq!(Location::parse_symbol('s'), Err("Invalid symbol"));
    }

    #[test]
    fn test_parse_grid_row() {
        let row = GridRow::parse(SAMPLE_INPUT_ROW);

        assert_eq!(
            row.0,
            vec![
                Location::Empty,
                Location::Empty,
                Location::PaperRoll,
                Location::PaperRoll,
                Location::Empty,
                Location::PaperRoll,
                Location::PaperRoll,
                Location::PaperRoll,
                Location::PaperRoll,
                Location::Empty,
            ]
        );
    }

    #[test]
    fn test_parse_grid() {
        let grid = Grid::parse(
            r"..@@.
@@@.@
@.@@.",
        )
        .unwrap();

        assert_eq!(
            grid.locations,
            vec![
                Location::Empty,
                Location::Empty,
                Location::PaperRoll,
                Location::PaperRoll,
                Location::Empty,
                Location::PaperRoll,
                Location::PaperRoll,
                Location::PaperRoll,
                Location::Empty,
                Location::PaperRoll,
                Location::PaperRoll,
                Location::Empty,
                Location::PaperRoll,
                Location::PaperRoll,
                Location::Empty,
            ]
        );
    }

    #[test]
    fn test_add_grid_row() {
        let row = GridRow::parse(SAMPLE_INPUT_ROW);
        let mut grid = Grid::new(SAMPLE_INPUT_ROW.len());

        grid.add_row(row);

        assert_eq!(
            grid.locations,
            vec![
                Location::Empty,
                Location::Empty,
                Location::PaperRoll,
                Location::PaperRoll,
                Location::Empty,
                Location::PaperRoll,
                Location::PaperRoll,
                Location::PaperRoll,
                Location::PaperRoll,
                Location::Empty,
            ]
        );
    }

    #[test]
    fn test_full_grid_directions() {
        let grid = Grid::parse(SAMPLE_INPUT_UPPER_FULL).unwrap();
        let grid_center = GridPosition {
            x: grid.width as i16 / 2,
            y: grid.width as i16 / 2,
        };

        assert_eq!(
            grid.get_neighbour(&grid_center, &GridDirection::Up),
            Some(Location::PaperRoll)
        );
        assert_eq!(
            grid.get_neighbour(&grid_center, &GridDirection::UpRight),
            Some(Location::PaperRoll)
        );
        assert_eq!(
            grid.get_neighbour(&grid_center, &GridDirection::Right),
            Some(Location::Empty)
        );
        assert_eq!(
            grid.get_neighbour(&grid_center, &GridDirection::RightDown),
            Some(Location::Empty)
        );
        assert_eq!(
            grid.get_neighbour(&grid_center, &GridDirection::Down),
            Some(Location::Empty)
        );
        assert_eq!(
            grid.get_neighbour(&grid_center, &GridDirection::DownLeft),
            Some(Location::Empty)
        );
        assert_eq!(
            grid.get_neighbour(&grid_center, &GridDirection::Left),
            Some(Location::Empty)
        );
        assert_eq!(
            grid.get_neighbour(&grid_center, &GridDirection::LeftUp),
            Some(Location::PaperRoll)
        );
    }

    #[test]
    fn test_empty_grid_directions() {
        let grid = Grid::parse(SAMPLE_INPUT_UPPER_EMPTY).unwrap();
        let grid_center = GridPosition {
            x: grid.width as i16 / 2,
            y: grid.width as i16 / 2,
        };

        assert_eq!(
            grid.get_neighbour(&grid_center, &GridDirection::Up),
            Some(Location::Empty)
        );
        assert_eq!(
            grid.get_neighbour(&grid_center, &GridDirection::UpRight),
            Some(Location::Empty)
        );
        assert_eq!(
            grid.get_neighbour(&grid_center, &GridDirection::Right),
            Some(Location::PaperRoll)
        );
        assert_eq!(
            grid.get_neighbour(&grid_center, &GridDirection::RightDown),
            Some(Location::PaperRoll)
        );
        assert_eq!(
            grid.get_neighbour(&grid_center, &GridDirection::Down),
            Some(Location::PaperRoll)
        );
        assert_eq!(
            grid.get_neighbour(&grid_center, &GridDirection::DownLeft),
            Some(Location::PaperRoll)
        );
        assert_eq!(
            grid.get_neighbour(&grid_center, &GridDirection::Left),
            Some(Location::PaperRoll)
        );
        assert_eq!(
            grid.get_neighbour(&grid_center, &GridDirection::LeftUp),
            Some(Location::Empty)
        );
    }

    #[test]
    fn test_grid_get_num_paper_roll_neighbours() {
        let grid = Grid::parse(SAMPLE_INPUT).unwrap();

        assert_eq!(
            grid.get_num_paper_roll_neighbours(&GridPosition { x: 7, y: 0 }),
            4
        );
        assert_eq!(
            grid.get_num_paper_roll_neighbours(&GridPosition { x: 6, y: 0 }),
            3
        );
        assert_eq!(
            grid.get_num_paper_roll_neighbours(&GridPosition { x: 0, y: 9 }),
            1
        );
    }

    #[test]
    fn get_index_from_position() {
        let grid = Grid::parse(SAMPLE_INPUT).unwrap();

        assert_eq!(grid.index_from_position(&GridPosition { x: 0, y: 0 }), 0);
        assert_eq!(grid.index_from_position(&GridPosition { x: 5, y: 4 }), 45);
        assert_eq!(grid.index_from_position(&GridPosition { x: 7, y: 3 }), 37);
        assert_eq!(grid.index_from_position(&GridPosition { x: 0, y: 2 }), 20);
        assert_eq!(grid.index_from_position(&GridPosition { x: 9, y: 9 }), 99);
        assert_eq!(grid.index_from_position(&GridPosition { x: 9, y: 0 }), 9);
    }

    #[test]
    fn get_position_from_index() {
        let grid = Grid::parse(SAMPLE_INPUT).unwrap();

        assert_eq!(grid.position_from_index(0), GridPosition { x: 0, y: 0 });
        assert_eq!(grid.position_from_index(45), GridPosition { x: 5, y: 4 });
        assert_eq!(grid.position_from_index(37), GridPosition { x: 7, y: 3 });
        assert_eq!(grid.position_from_index(20), GridPosition { x: 0, y: 2 });
        assert_eq!(grid.position_from_index(99), GridPosition { x: 9, y: 9 });
        assert_eq!(grid.position_from_index(9), GridPosition { x: 9, y: 0 });
    }

    #[test]
    fn test_num_accessible() {
        let grid = Grid::parse(SAMPLE_INPUT).unwrap();

        let num_accessible = grid.get_num_accessible();

        assert_eq!(num_accessible, 13);
    }

    #[test]
    fn test_render_to_string() {
        let grid = Grid::parse(SAMPLE_INPUT).unwrap();

        assert_eq!(
            grid.render_to_string(),
            r"..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x."
        );
    }

    #[test]
    fn test_grid_remove_once() {
        let mut grid = Grid::parse(SAMPLE_INPUT).unwrap();

        let num_removed = grid.remove_accessible();

        assert_eq!(num_removed, 13);
        assert_eq!(
            grid.render_to_string(),
            r".......x..
.@@.x.x.@x
x@@@@...@@
x.@@@@..x.
.@.@@@@.x.
.x@@@@@@.x
.x.@.@.@@@
..@@@.@@@@
.x@@@@@@@.
....@@@..."
        );
    }

    #[test]
    fn test_grid_remove_thrice() {
        let mut grid = Grid::parse(SAMPLE_INPUT).unwrap();

        let num_removed = grid.remove_accessible();
        assert_eq!(num_removed, 13);

        let num_removed = grid.remove_accessible();
        assert_eq!(num_removed, 12);

        let num_removed = grid.remove_accessible();
        assert_eq!(num_removed, 7);
    }

    #[test]
    fn test_grid_remove_all() {
        let mut grid = Grid::parse(SAMPLE_INPUT).unwrap();

        let mut total_removed = grid.remove_until_impossible();

        assert_eq!(total_removed, 43);
    }
}
