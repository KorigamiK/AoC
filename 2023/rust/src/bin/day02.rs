// use regex::Regex;
use std::io::{self, Read};

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once(": ").unwrap();
            let (mut red, mut blue, mut green) = (0, 0, 0);
            rest.split("; ").for_each(|set| {
                set.split(", ").for_each(|cube| {
                    let (digit, color) = cube.split_once(' ').unwrap();
                    let digit = digit.parse::<u32>().unwrap();
                    match color {
                        "blue" => blue = blue.max(digit),
                        "green" => green = green.max(digit),
                        "red" => red = red.max(digit),
                        _ => {}
                    }
                });
            });
            blue * green * red
        })
        .sum::<u32>()
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let lines: Vec<_> = buffer
        .split('\n')
        .filter(|inp| inp != &"")
        .map(|inp| inp.to_string())
        .collect();

    let mut winners: Vec<usize> = vec![];

    for line in lines {
        let game_start = line.find(':').unwrap();
        let game_id = &line[0..game_start];
        let game_id: Vec<_> = game_id.split(' ').collect();
        let game_id = game_id[1].parse::<usize>().unwrap();

        let games = &line[game_start + 2..];

        // split the line from game_idx + 1 till the end on ;
        let possible = games.trim_end().split(';').all(|game| {
            let game = game.trim();
            let possible = game.split(',').all(|g| {
                let g = g.trim();
                let (cubes, color) = g.split_once(' ').unwrap();
                let cubes = cubes.parse::<usize>().unwrap();
                match color {
                    "blue" => cubes <= 14,
                    "green" => cubes <= 13,
                    "red" => cubes <= 12,
                    _ => false,
                }
            });
            possible
        });

        if possible {
            // println!("WINNER: {game_id}");
            winners.push(game_id);
        }
    }

    println!("{}", winners.iter().sum::<usize>());
    println!("part2: {}", part2(&buffer));

    Ok(())
}
