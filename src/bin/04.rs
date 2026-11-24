use hashbrown::HashSet;
use itertools::Itertools;

advent_of_code::solution!(4);

pub fn test_passphrase_validity_part1(input: &str) -> bool {
    let words: Vec<&str> = input.split(" ").collect();
    let words_len = words.len();

    let hash: HashSet<&str> = HashSet::from_iter(words.into_iter());

    hash.len() == words_len
}

pub fn test_passphrase_validity_part2(input: &str) -> bool {
    let words: Vec<String> = input
        .split(" ")
        .map(|w| w.chars().sorted_unstable().collect::<String>())
        .collect();
    let words_len = words.len();

    let hash: HashSet<String> = HashSet::from_iter(words.into_iter());

    hash.len() == words_len
}
pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .trim()
            .lines()
            .filter(|l| test_passphrase_validity_part1(l))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .trim()
            .lines()
            .filter(|l| test_passphrase_validity_part2(l))
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert!(test_passphrase_validity_part1("aa bb cc"));
        assert!(!test_passphrase_validity_part1("aa bb aa"));
    }

    #[test]
    fn test_part_two() {
        assert!(test_passphrase_validity_part2("aa bb cc"));
        assert!(!test_passphrase_validity_part2("ac bb ca"));
    }
}
