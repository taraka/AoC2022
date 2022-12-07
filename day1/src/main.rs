use std::io::{self, Read};

fn main() -> io::Result<()>  {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    println!("Part1: {}", part_1_and_2(&buffer, 1));
    println!("Part2: {}", part_1_and_2(&buffer, 3));

    Ok(())
}

fn part_1_and_2(input: &str, s: usize) -> u64 {
    let mut sums = vec![];
    let mut curr: u64 = 0;
    for l in input.lines() {
        if l.len() == 0 {
            sums.push(curr);
            curr = 0;
        } else {
            curr += u64::from_str_radix(l, 10).unwrap();
        }
    }
    sums.push(curr);
    
    sums.sort();
    sums.iter().rev().take(s).sum()
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_part1() {
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

        assert_eq!(part_1_and_2(&input, 1), 24000);
        assert_eq!(part_1_and_2(&input, 3), 45000);
    }

}