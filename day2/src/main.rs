use std::io::{self, Read};

fn main() -> io::Result<()>  {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    println!("Part1: {}", part_1(&buffer));
    println!("Part2: {}", part_2(&buffer));


    Ok(())
}

fn part_1(input: &str) -> i64 {
    input
        .lines()
        .map(line_score_1)
        .sum()
}

fn part_2(input: &str) -> i64 {
    input
        .lines()
        .map(line_score_2)
        .sum()
}

//1 for Rock, 2 for Paper, and 3 for Scissors

fn line_score_1(line: &str) -> i64 {
    let bytes = line.as_bytes();
    let op = bytes[0] as i64 - 64;
    let me = bytes[2] as i64 - 87;
    
    let score = match (op - me).abs() {
        0 => 3,
        1 => if op > me { 0 } else { 6 } 
        _ => if op < me { 0 } else { 6 } 
    };

    score + me
}

fn line_score_2(line: &str) -> i64 {
    let bytes = line.as_bytes();
    let op = bytes[0] as i64 - 64;
    let score = (bytes[2] as i64 - 88) * 3 ;
    
    let me = match score {
        // loose
        0 => ((op - 2).rem_euclid(3)) + 1,
        3 => op,
        // win
        6 => (op % 3) + 1,
        _ => panic!("Not reachable")
    };

    if me == 0 {
        println!("{}, {} ,{}, {:?}, {}", line, score, me, bytes, op);
    }
    score + me
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_part1() {
        let input = r#"A Y
B X
C Z"#;

        assert_eq!(part_1(&input), 15);
        assert_eq!(part_2(&input), 12);
    }

}