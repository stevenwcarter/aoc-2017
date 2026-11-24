advent_of_code::solution!(5);

pub fn solve(input: &str, part2: bool) -> Option<u64> {
    let mut values: Vec<isize> = input
        .trim()
        .lines()
        .map(|l| str::parse::<isize>(l).expect("Not a valid number"))
        .collect();
    let mut index: usize = 0;
    let mut steps: u64 = 0;

    loop {
        let jump = values.get(index).copied();
        if jump.is_none() {
            return Some(steps);
        }
        let jump = jump.unwrap();
        if part2 && jump >= 3 {
            values[index] -= 1;
        } else {
            values[index] += 1;
        }

        let result = index.checked_add_signed(jump);
        if result.is_none() {
            return Some(steps);
        } else {
            index = result.unwrap();
        }

        steps += 1;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
