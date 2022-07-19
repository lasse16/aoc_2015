pub fn part_one(input: &str) -> u32 {
    let mut compound_square_feet = 0;
    for line in input.lines() {
        let dimensions: Vec<u32> = line.split('x').map(|x| x.parse().unwrap()).collect();
        let side_a = dimensions[0] * dimensions[1];
        let side_b = dimensions[1] * dimensions[2];
        let side_c = dimensions[2] * dimensions[0];
        let sides = [side_a, side_b, side_c];
        let min_side = sides.iter().min().unwrap();
        compound_square_feet += sides.iter().fold(0, |acc, side| acc + 2 * side);
        compound_square_feet += min_side;
    }
    compound_square_feet
}

pub fn part_two(input: &str) -> u32 {
    let mut ribbon_need = 0;
    for line in input.lines() {
        let mut dimensions: Vec<u32> = line
            .split('x')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>();
        dimensions.sort();
        let volume = dimensions[0] * dimensions[1] * dimensions[2];
        ribbon_need += (dimensions[0] + dimensions[1]) * 2;
        ribbon_need += volume;
    }
    ribbon_need
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 2), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_one(&input), 101);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_two(&input), 48);
    }
}
