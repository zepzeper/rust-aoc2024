use std::collections::HashMap;

// (Width, Height)
type Coordinate = (i32, i32);

const CHECKMAS: &str = "MAS";
const BAD_CHAR: &char = &'.';

pub enum Direction {
    Up,
    UpRight,
    UpLeft,
    Down,
    DownRight,
    DownLeft,
    Right,
    Left
}

pub fn is_this_xmas(word_grid: &HashMap::<Coordinate, char>, coordinates: Vec<Coordinate>) -> bool {
    let mut mas = "".to_string();

    for coordinate in coordinates.iter() {
        mas.push(word_grid.get(coordinate).unwrap_or(BAD_CHAR).to_owned());
    }

    mas == CHECKMAS
}

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let lines = input.trim().lines().collect::<Vec<&str>>();
    let mut word_grid = HashMap::<Coordinate, char>::new();
    let mut x_words = Vec::<Coordinate>::new();
    let mut a_words = Vec::<Coordinate>::new();

    for (height, line) in lines.iter().enumerate() {
        for (width, char) in line.chars().enumerate() {
            word_grid.insert((width as i32, height as i32), char);
            if char == 'A' {
                a_words.push((width as i32, height as i32));
            }
            if char == 'X' {
                x_words.push((width as i32, height as i32));
            }
        }
    }

    let directions = [
        Direction::Up,
        Direction::UpRight,
        Direction::UpLeft,
        Direction::Down,
        Direction::DownLeft,
        Direction::DownRight,
        Direction::Right,
        Direction::Left,
    ];

    let mut xms = 0;

    for coordinate in x_words.iter() {
        for direction in &directions {
            if is_this_xmas(&word_grid, generate_coordinates_part1(coordinate, direction)) {
                xms += 1;
            }
        }
    }

    Ok(xms.to_string())
}

fn generate_coordinates_part1(origin: &Coordinate, direction: &Direction) -> Vec<Coordinate> {
    match direction {
        Direction::Up => {
            vec![
                (origin.0, origin.1-1),
                (origin.0, origin.1-2),
                (origin.0, origin.1-3),
            ]
        }
        Direction::UpRight => {
            vec![
                (origin.0+1, origin.1-1),
                (origin.0+2, origin.1-2),
                (origin.0+3, origin.1-3),
            ]
        }
        Direction::Right => {
            vec![
                (origin.0+1, origin.1),
                (origin.0+2, origin.1),
                (origin.0+3, origin.1),
            ]
        }
        Direction::DownRight => {
            vec![
                (origin.0+1, origin.1+1),
                (origin.0+2, origin.1+2),
                (origin.0+3, origin.1+3),
            ]
        }
        Direction::Down => {
            vec![
                (origin.0, origin.1+1),
                (origin.0, origin.1+2),
                (origin.0, origin.1+3),
            ]
        }
        Direction::DownLeft => {
            vec![
                (origin.0-1, origin.1+1),
                (origin.0-2, origin.1+2),
                (origin.0-3, origin.1+3),
            ]
        }
        Direction::Left => {
            vec![
                (origin.0-1, origin.1),
                (origin.0-2, origin.1),
                (origin.0-3, origin.1),
            ]
        }
        Direction::UpLeft => {
            vec![
                (origin.0-1, origin.1-1),
                (origin.0-2, origin.1-2),
                (origin.0-3, origin.1-3),
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn std::error::Error>> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
