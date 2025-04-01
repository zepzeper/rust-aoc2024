use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_forward(&self, pos: (usize, usize)) -> (usize, usize) {
        let (row, col) = pos;

        match self {
            Direction::Up => (row.saturating_sub(1), col),
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col.saturating_sub(1)),
        }
    }
}

#[derive(Debug)]
struct Grid {
    obstacles: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
    guard_pos: (usize, usize),
    guard_dir: Direction,
    visited: HashSet<(usize, usize)>
}

impl Grid {
    fn next_step(&mut self) -> bool {
        let next_pos = self.guard_dir.move_forward(self.guard_pos);

        if !self.is_within_bound(next_pos) {
            return false;
        }

        if self.is_obstacle(next_pos) {
            self.guard_dir = self.guard_dir.turn_right();
        } else {

            if self.will_fall_off(next_pos) {
                self.visited.insert(next_pos);
                return false;
            }

            self.guard_pos = next_pos;
            self.visited.insert(next_pos);
        }

        true
    }

    fn will_fall_off(&self, pos: (usize, usize)) -> bool {
        let (row, col) = pos;

        match self.guard_dir {
            Direction::Up => row == 0,
            Direction::Down => row >= self.height - 1,
            Direction::Left => col == 0,
            Direction::Right => col >= self.width - 1,
        }
    }

    fn is_obstacle(&self, pos: (usize, usize)) -> bool {
        self.obstacles.contains(&pos)
    }
    
    fn is_within_bound(&self, pos: (usize, usize)) -> bool {
        let (row, col) = pos;

        row < self.height && col < self.width
    }

    fn simulate(&mut self) -> usize {
        while self.next_step() {}
        self.visited.len()
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines.first().map(|l| l.len()).unwrap();

        let mut obstacles = HashSet::new();
        let mut guard_dir = Direction::Up;
        let mut guard_pos = (0,0);
        let mut visited = HashSet::new();

        for (row, line) in lines.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        obstacles.insert((row, col));
                    }
                    '^' => {
                        guard_dir = Direction::Up;
                        visited.insert((row, col));
                        guard_pos = (row, col);
                    },
                    '>' => {
                        guard_dir = Direction::Right;
                        visited.insert((row, col));
                        guard_pos = (row, col);
                    },
                    'v' => {
                        guard_dir = Direction::Down;
                        visited.insert((row, col));
                        guard_pos = (row, col);
                    },
                    '<' => {
                        guard_dir = Direction::Left;
                        visited.insert((row, col));
                        guard_pos = (row, col);
                    },
                    _ => {}
                }
            }
        }

        Self {
            obstacles, 
            width,
            height,
            guard_pos,
            guard_dir,
            visited
        }
    }
    
}

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {

    let mut grid = Grid::from(input);

    let distinct_postitions = grid.simulate();
    
    Ok(distinct_postitions.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn std::error::Error>> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
