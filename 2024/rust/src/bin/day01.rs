use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();
    buffer.split('\n').filter(|inp| inp != &"").for_each(|inp| {
        let mut nums = inp.split_whitespace();
        l1.push(nums.next().unwrap().parse().unwrap());
        l2.push(nums.next().unwrap().parse().unwrap());
    });
    let freq: std::collections::HashMap<i32, i32> =
        l2.iter()
            .fold(std::collections::HashMap::new(), |mut acc, &x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            });
    let mut ans = 0i32;
    (0..l1.len()).for_each(|i| {
        ans += l1[i] * freq.get(&l1[i]).unwrap_or(&0);
    });
    println!("{ans}");
    Ok(())
}
