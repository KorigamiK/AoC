use std::io::{self, Read};

trait InBounds {
    fn in_bounds(&self, x: i32, y: i32) -> bool;
}

impl InBounds for Vec<Vec<char>> {
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.len() as i32 && y >= 0 && y < self[0].len() as i32
    }
}

fn part_1(input: &str) {
    let lines: Vec<_> = input.split('\n').collect();
    let mut array: Vec<Vec<char>> = vec![];
    for line in lines {
        let cols: Vec<_> = line.chars().collect();
        array.push(cols);
    }
    let mut ans = 0;
    for x in 0..array.len() {
        let mut s: i32 = -1;
        for y in 0..array.first().unwrap().len() {
            match array[x][y].is_digit(10) {
                true => {
                    if s == -1 {
                        s = y as i32;
                    }
                }
                false => {
                    if s != -1 {
                        let number = array[x][s as usize..y]
                            .iter()
                            .collect::<String>()
                            .parse::<i32>()
                            .unwrap();

                        let left = if array.in_bounds(x as i32, s - 1) {
                            array[x][s as usize - 1] == '.'
                        } else {
                            true
                        };

                        let right = if array.in_bounds(x as i32, y as i32) {
                            array[x][y] == '.'
                        } else {
                            true
                        };

                        let up = if array.in_bounds(x as i32 - 1, s) {
                            let mut ans = true;
                            for i in array[x - 1][s as usize..y].iter() {
                                ans &= i == &'.';
                            }
                            ans
                        } else {
                            true
                        };

                        let down = if array.in_bounds(x as i32 + 1, s as i32) {
                            let mut ans = true;
                                println!("{x} {y}");
                            for i in array[x + 1][s as usize..y].iter() {
                                ans &= i == &'.';
                            }
                            ans
                        } else {
                            true
                        };

                        println!("number: {number} s: {s} y: {y}");
                        let diagonally = {
                            let mut ans = true;
                            let y = y - 1;
                            let d: Vec<(i32, i32)> = vec![
                                (x as i32 + 1, s - 1),
                                (x as i32 - 1, s - 1),
                                (x as i32 + 1, y as i32 + 1),
                                (x as i32 - 1, y as i32 + 1),
                            ];
                            for (nx, ny) in d {
                                if array.in_bounds(nx, ny) {
                                    // println!(
                                    //     "nx: {nx} ny: {ny} {}",
                                    //     array[nx as usize][ny as usize]
                                    // );
                                    ans &= array[nx as usize][ny as usize] == '.';
                                }
                            }
                            ans
                        };

                        // println!(
                        //     "left: {}, right: {}, up: {}, down: {} diagonally: {}",
                        //     left, right, up, down, diagonally
                        // );

                        if diagonally && up && down && left && right {
                            // not part of the numbers
                        } else {
                            // part of the numbers
                            ans += number;
                        }
                    }
                    s = -1;
                }
            }
        }
    }
    println!("{ans}");
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    part_1(&buffer);
    Ok(())
}
