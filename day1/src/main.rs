use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let masses: Vec<i32> = BufReader::new(File::open("input1.txt").expect("input not found"))
        .lines() // all the individual lines
        .map(|line| {
            line.expect("could not parse line") // if the line is not valid
                .parse() //convert to what I annotated as (i32)
                .expect("could not parse number") // if the i32 is not valid
        })
        .collect();

    let sum: i32 = masses.iter().map(fuel_needed).sum();

    println!("Total fuel needed: {}", sum);
}

fn fuel_needed(mass: &i32) -> i32 {
    let fuel = mass / 3 - 2;

    if (fuel) > 0 {
        fuel + fuel_needed(&fuel) // recursively calculate the total fuel
    } else {
        0
    }
}
