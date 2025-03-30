use std::collections::HashMap;

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {

    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut left: Vec<i32>= vec![];
    let mut right: Vec<i32> = vec![];
    let mut total = 0;
    
    for line in input.lines() {
        let mut items = line.split_whitespace();

        left.push(
            items.next().unwrap().parse::<i32>().unwrap()
        );
        right.push(
            items.next().unwrap().parse::<i32>().unwrap()
        );
    }

    for x in right.iter() {
        map.entry(*x).and_modify(|e| *e += 1).or_insert(1); // Count occurences
    }

    for y in left.iter() {
        total += y * map.get(y).unwrap_or(&0);
    }


    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn std::error::Error>> {
        let input ="3   4
                    4   3
                    2   5
                    1   3
                    3   9
                    3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
