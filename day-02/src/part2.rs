fn check_sequence(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return false;
    }

    let increasing = numbers[1] > numbers[0];

    for i in 1..numbers.len() {
        if (increasing && numbers[i] <= numbers[i - 1]) || 
        (!increasing && numbers[i] >= numbers[i - 1]) {
            return false;
        }

        let diff = if increasing {
            numbers[i] - numbers[i - 1]
        } else {
            numbers[i - 1] - numbers[i]
        };

        if !(1..=3).contains(&diff) {
            return false;
        }
    }

    true
}

fn is_safe(numbers: &[i32]) -> bool {

    if check_sequence(numbers) {
        return true;
    }

    for i in 0..numbers.len() {
        let mut modified = numbers.to_vec();
        modified.remove(i);

        if check_sequence(&modified) {
            return true;
        }
    }

    false
}

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut safe_count = 0;

    for line in input.lines() {
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        if is_safe(&numbers) {
            safe_count += 1;
        }

    }

    Ok(safe_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn std::error::Error>> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
