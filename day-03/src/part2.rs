use std::fmt::Display;

use regex::Regex;

#[derive(Debug)]
struct Mul {
   x: i32,
   y: i32 
}

impl Mul {
    pub fn new(x: i32, y: i32) -> Self {
        Mul { x, y }
    }
    
    pub fn value(&self) -> i32 {
        self.x * self.y
    }
}

impl Display for Mul {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn parse_mul(input: &str) -> Vec<Mul> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Invalid regex pattern");
    let do_re = Regex::new(r"do\(\)").expect("Invalid regex pattern");
    let dont_re = Regex::new(r"don\'t\(\)").expect("Invalid regex pattern");

    let mut result = Vec::new();

    let mut enabled = true;
    let mut pos = 0;

    while pos < input.len() {
        let remaining = &input[pos..];

        if let Some(m) = do_re.find(remaining) {
            if m.start() == 0 {
                enabled = true;
                pos += m.end();
                continue;
            }
        }

        if let Some(m) = dont_re.find(remaining) {
            if m.start() == 0 {
                enabled = false;
                pos += m.end();
                continue;
            }
        }

        if enabled {
            if let Some(cap) = re.captures(remaining) {
                if cap.get(0).unwrap().start() == 0 {
                    let x = cap[1].parse::<i32>().unwrap();
                    let y = cap[2].parse::<i32>().unwrap();
                    result.push(Mul::new(x, y));
                    pos += cap.get(0).unwrap().end();
                    continue;
                }
            }
        }

        pos += 1;
    }

    result
}

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let muls = parse_mul(input);

    dbg!(&muls);

    let sum: i32 = muls.iter().map(|v| v.value()).sum();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn std::error::Error>> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
