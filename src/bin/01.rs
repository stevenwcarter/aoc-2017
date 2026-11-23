advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let digits: Vec<u8> = input.trim().chars().map(|c| c as u8 - b'0').collect();

    let final_comparison = vec![*digits.last().unwrap(), *digits.first().unwrap()];
    Some(
        digits
            .windows(2)
            .chain(std::iter::once(final_comparison.as_slice()))
            .filter(|v| v[0] == v[1])
            .map(|n| n[0] as u64)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let digits: Vec<u8> = input.trim().chars().map(|c| c as u8 - b'0').collect();
    let len = digits.len();
    let offset = len / 2;

    Some(
        (0..len)
            .map(|index| (digits[index], digits[(index + offset) % len]))
            .filter(|(a, b)| a == b)
            .map(|(a, _)| a as u64)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("1221"), Some(3));
        assert_eq!(part_one("1111"), Some(4));
        assert_eq!(part_one("1234"), Some(0));
        assert_eq!(part_one("91212129"), Some(9));
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("1212"), Some(6));
        assert_eq!(part_two("1221"), Some(0));
        assert_eq!(part_two("123425"), Some(4));
        assert_eq!(part_two("123123"), Some(12));
        assert_eq!(part_two("12131415"), Some(4));
    }
}
