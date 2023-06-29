use std::fs;
use std::io::{self, Read};

fn parse_elve(input: &str) -> i32 {
    input.lines().map(|line| line.parse::<i32>().unwrap()).sum()
}

fn main() -> io::Result<()> {
    let mut file = fs::File::open("../data/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut elves: Vec<i32> = contents
        .split("\n\n")
        .map(parse_elve)
        .collect();

        elves.sort_unstable_by(|a, b| b.cmp(a));
        elves.truncate(3);

    println!("Top score: {}", elves[0]);
    println!("3 top scores sum: {}", elves.iter().sum::<i32>());

    Ok(())
}
