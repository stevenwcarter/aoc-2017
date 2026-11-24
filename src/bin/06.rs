use hashbrown::{HashMap, HashSet};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let mut memory: HashSet<Vec<u8>> = HashSet::new();
    let mut banks: Vec<u8> = input
        .trim()
        .split("\t")
        .map(|n| str::parse::<u8>(n).expect("expected a number"))
        .collect();

    while memory.insert(banks.clone()) {
        let max = banks.iter().max().copied().unwrap();
        let index = banks.iter().position(|&l| l == max).unwrap();
        banks[index] = 0;

        (0..max).for_each(|n| {
            let new_index = (index + 1 + n as usize) % banks.len();
            banks[new_index] += 1;
        });
    }
    Some(memory.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut memory: HashMap<Vec<u8>, usize> = HashMap::new();
    let mut banks: Vec<u8> = input
        .trim()
        .split("\t")
        .map(|n| str::parse::<u8>(n).expect("expected a number"))
        .collect();

    loop {
        let result = memory.insert(banks.clone(), memory.len());
        if result.is_none() {
            let max = banks.iter().max().copied().unwrap();
            let index = banks.iter().position(|&l| l == max).unwrap();
            banks[index] = 0;

            (0..max).for_each(|n| {
                let new_index = (index + 1 + n as usize) % banks.len();
                banks[new_index] += 1;
            });
        } else {
            return Some(memory.len() - result.unwrap());
        }
    }

    // println!(
    //     "FINAL: {} - {}",
    //     memory.len(),
    //     memory.get(&banks).copied().unwrap()
    // );
    // Some(memory.len() - old)
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
        assert_eq!(result, Some(4));
    }
}
