advent_of_code::solution!(1);
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u64> {
    let digits: Vec<u8> = input.trim().chars().map(|c| c as u8 - b'0').collect();

    Some(
        digits
            .iter()
            .circular_tuple_windows()
            .filter(|(a, b)| a == b)
            .map(|(a, _)| *a as u64)
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
