use std::collections::HashMap;

// (Width, Height)
type Coordinate = (i32, i32);

const BAD_CHAR: &char = &'.';
const SM: &str = "SM";
const MS: &str = "MS";

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

pub fn is_this_x_mas(word_grid: &mut HashMap::<Coordinate, char>, coordinates: [[Coordinate;2];2]) -> bool {
    for coord_pair in coordinates.iter() {
        let mut mas = "".to_string();

        for coordinate in coord_pair.iter() {
            mas.push(word_grid.get(coordinate).unwrap_or(BAD_CHAR).to_owned());
        }

        if mas != MS && mas != SM {
            return false;
        }
    }

    true
}

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let lines = input.trim().lines().collect::<Vec<&str>>();
    let mut word_grid = HashMap::<Coordinate, char>::new();
    let mut a_words = Vec::<Coordinate>::new();

    for (height, line) in lines.iter().enumerate() {
        for (width, char) in line.chars().enumerate() {
            word_grid.insert((width as i32, height as i32), char);
            if char == 'A' {
                a_words.push((width as i32, height as i32));
            }
        }
    }

    let mut ms = 0;

    for coordinate in a_words.iter() {
        let coords = generate_coordinates_part2(coordinate);
        if is_this_x_mas(&mut word_grid, coords) {
            ms += 1;
        }
    }

    Ok(ms.to_string())
}

fn generate_coordinates_part2(origin: &Coordinate) -> [[Coordinate;2];2] {
    [
        [
            (origin.0 - 1, origin.1 - 1), // Top left
            (origin.0 + 1, origin.1 + 1)  // Bottom right
        ],
        [
            (origin.0 + 1, origin.1 - 1), // Top right
            (origin.0 - 1, origin.1 + 1)  // Bottom left
        ]
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn std::error::Error>> {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
