pub fn part_one(input: &str) -> i16 {
    let mut floor: i16 = 0;
    for character in input.chars() {
        if character == '(' {
            floor += 1
        }
        if character == ')' {
            floor -= 1
        }
    }
    floor
}

pub fn part_two(input: &str) -> u32 {
    let mut floor: i16 = 0;
    let mut step: u32 = 0;
    for character in input.chars() {
        step += 1;
        if character == '(' {
            floor += 1
        }
        if character == ')' {
            floor -= 1
        }
        if floor == -1 {
            return step;
        }
    }
    step
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 1), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 1);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 1);
        assert_eq!(part_two(&input), 0);
    }
}
