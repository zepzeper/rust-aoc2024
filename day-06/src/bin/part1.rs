use day_06::part1::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let file = include_str!("../../input1.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}
