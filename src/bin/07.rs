use hashbrown::HashMap;

advent_of_code::solution!(7);

#[derive(Debug)]
pub struct Tower<'a> {
    pub id: &'a str,
    pub weight: u32,
    pub sub_towers: Option<Vec<&'a str>>,
}

impl<'a> Tower<'a> {
    pub fn parse(input: &'a str) -> Self {
        let (id, weight) = input
            .split_once(" (")
            .expect("All lines should have an id and weight");
        let (weight, remainder) = weight
            .split_once(")")
            .expect("All weights should have a closing parentheses");
        let weight = str::parse::<u32>(weight).expect("Weight was not a number");

        let sub_towers = if remainder.contains("->") {
            Some(
                remainder
                    .strip_prefix(" -> ")
                    .unwrap()
                    .split(", ")
                    .collect(),
            )
        } else {
            None
        };

        Self {
            id,
            weight,
            sub_towers,
        }
    }
}

pub fn find_base(towers: &[Tower]) -> Option<String> {
    let counts = towers
        .iter()
        .fold(HashMap::<&str, u32>::new(), |mut acc, t| {
            acc.entry(t.id).or_insert(0);
            if let Some(sub_towers) = t.sub_towers.as_ref() {
                sub_towers.iter().for_each(|id| {
                    *acc.entry(id).or_insert(0) += 1;
                });
            }
            acc
        });

    let result = counts.iter().find(|(_, v)| **v == 0).unwrap();

    Some(String::from(*result.0))
}

/// Total weight of the tower rooted at `base`: its own weight plus everything above it.
pub fn weigh_tower<'a>(
    towers: &HashMap<&'a str, Tower<'a>>,
    cache: &mut HashMap<&'a str, u32>,
    base: &str,
) -> u32 {
    if let Some(weight) = cache.get(base) {
        return *weight;
    }
    let tower = towers.get(base).expect("Base should exist..");
    let total = tower.weight
        + tower
            .sub_towers
            .iter()
            .flatten()
            .map(|s| weigh_tower(towers, cache, s))
            .sum::<u32>();
    cache.insert(tower.id, total);
    total
}

pub fn find_wrong_weight<'a>(
    towers: &HashMap<&'a str, Tower<'a>>,
    cache: &mut HashMap<&'a str, u32>,
    base: &str,
    target: Option<u32>,
) -> Option<u32> {
    let tower = towers.get(base).expect("base didn't exist");

    let mut by_weight = HashMap::<u32, Vec<&str>>::new();
    for sub in tower.sub_towers.iter().flatten() {
        by_weight
            .entry(weigh_tower(towers, cache, sub))
            .or_default()
            .push(sub);
    }

    // The children disagree, so the wrong one is somewhere inside the odd one
    // out. Keep descending, telling it what its siblings weigh.
    if by_weight.len() > 1 {
        let (_, odd) = by_weight.iter().find(|(_, v)| v.len() == 1)?;
        let (&good, _) = by_weight.iter().find(|(_, v)| v.len() > 1)?;
        return find_wrong_weight(towers, cache, odd[0], Some(good));
    }

    // The children all agree (or there are none), yet this tower as a whole
    // is off from `target` - so this program itself is the wrong one.
    let total = weigh_tower(towers, cache, base);
    Some(tower.weight + target? - total)
}

pub fn part_one(input: &str) -> Option<String> {
    let towers: Vec<Tower> = input.lines().map(|l| Tower::parse(l)).collect();

    find_base(&towers)
}

pub fn part_two(input: &str) -> Option<u32> {
    let towers: Vec<Tower> = input.lines().map(|l| Tower::parse(l)).collect();

    let base = find_base(&towers).expect("could not solve for base");

    let towers: HashMap<&str, Tower> = HashMap::from_iter(towers.into_iter().map(|t| (t.id, t)));
    let mut cache: HashMap<&str, u32> = HashMap::new();

    find_wrong_weight(&towers, &mut cache, base.as_str(), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&data);
        assert_eq!(result.as_deref(), Some("tknk"));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(60));
    }
}
