use std::{cmp, collections::HashMap};

#[derive(Debug, PartialEq)]
enum RuleOrder {
    Before,
    After,
}

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {

    let parts: Vec<&str> = input.split("\n\n").collect();

    let rules_section: HashMap<u32, HashMap<u32, RuleOrder>> = parts[0].lines()
        .filter(|line| !line.is_empty())
        .fold(HashMap::new(), |mut acc, line| {
            let mut pair = line.split("|");
            let first = pair.next().unwrap().parse::<u32>().unwrap();
            let second = pair.next().unwrap().parse::<u32>().unwrap();

            acc.entry(first).or_default().insert(second, RuleOrder::After);
            acc.entry(second).or_default().insert(first, RuleOrder::Before);

            acc
    });

    let updates_section: Vec<Vec<u32>> = parts[1].lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(",").map(|dig| {dig.parse::<u32>().unwrap()}).collect()
        }).collect();

    let mut sum = 0;

    'update_loop: for update in updates_section.iter() {
        for i in 0..update.len() {
            for j in i+1..update.len() {

                let page_i = update[i];
                let page_j = update[j];

                if let Some(relations) = rules_section.get(&page_i) {
                    if let Some(order) = relations.get(&page_j) {
                        if *order == RuleOrder::Before {
                            sum += order_pages(update, &rules_section);
                            continue 'update_loop;
                        }
                    }
                }

                if let Some(relations) = rules_section.get(&page_j) {
                    if let Some(order) = relations.get(&page_i) {
                        if *order == RuleOrder::After {
                            sum += order_pages(update, &rules_section);
                            continue 'update_loop;
                        }
                    }
                }
            }
        }

        //let middle_idx = update.len() / 2;
        //let middle_page = update[middle_idx];
        //sum += middle_page;
    }

    Ok(sum.to_string())
}

fn order_pages(update: &Vec<u32>, rules: &HashMap<u32, HashMap<u32, RuleOrder>>) -> u32 {

    let mut order_page = update.clone();

    order_page.sort_by(|&a, &b| {
        if let Some(relation_a) = rules.get(&a) {
            if let Some(order) = relation_a.get(&b) {
                match order {
                    RuleOrder::Before => return cmp::Ordering::Less,
                    RuleOrder::After => return cmp::Ordering::Greater
                }
            }
        }

        if let Some(relation_b) = rules.get(&b) {
            if let Some(order) = relation_b.get(&a) {
                match order {
                    RuleOrder::Before => return cmp::Ordering::Greater,
                    RuleOrder::After => return cmp::Ordering::Less
                }
            }
        }

        cmp::Ordering::Equal
    });


    let middle_idx = order_page.len() / 2;

    order_page[middle_idx]

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn std::error::Error>> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("123", process(input)?);
        Ok(())
    }
}
