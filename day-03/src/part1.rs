use regex::Regex;

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

fn parse_mul(input: &str) -> Vec<Mul> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Incorrect regex...");

    re.captures_iter(input)
        .map(|cap| {
            let x = cap[1].parse::<i32>().unwrap();
            let y = cap[2].parse::<i32>().unwrap();
            Mul::new(x, y)
        })
        .collect()
}

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let muls = parse_mul(input);
    let sum: i32 = muls.iter().map(|v| v.value()).sum();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn std::error::Error>> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
