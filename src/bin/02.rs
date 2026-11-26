advent_of_code::solution!(2);
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(build_vec)
        .map(|l| l.iter().max().unwrap() - l.iter().min().unwrap())
        .sum::<u32>()
        .into()
}

fn build_vec(input: &str) -> Vec<u32> {
    input
        .split("\t")
        .map(|c| str::parse::<u32>(c).unwrap())
        .collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(build_vec)
        .map(|l| {
            let combo = l
                .iter()
                .combinations(2)
                .find(|n| n[0].max(n[1]) % n[0].min(n[1]) == 0)
                .unwrap();

            combo[0].max(combo[1]) / combo[0].min(combo[1])
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));
    }
}
