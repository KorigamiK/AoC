use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let lines: Vec<_> = buffer
        .split('\n')
        .filter(|inp| inp != &"")
        .map(|inp| inp.to_string())
        .collect();

    let numbers_only: Vec<_> = lines
        .iter()
        .map(|inp| {
            inp.chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<char>>()
        })
        .collect();

    let sum = numbers_only.into_iter().fold(0, |acc, x| {
        // sum the first
        eprintln!("{} {}", x.first().unwrap(), x.last().unwrap());
        acc + x.first().unwrap().to_digit(10).unwrap() + x.last().unwrap().to_digit(10).unwrap()
    });

    println!("{}", sum);

    Ok(())
}
