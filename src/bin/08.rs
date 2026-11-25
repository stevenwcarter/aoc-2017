use hashbrown::HashMap;

advent_of_code::solution!(8);

// b inc 5 if a > 1
// a inc 1 if b < 5
// c dec -10 if a >= 1
// c inc -20 if c == 10
// <=
// !=

pub fn check_conditional(a: i32, conditional: &str, b: i32) -> bool {
    match conditional {
        ">" => a > b,
        "<" => a < b,
        ">=" => a >= b,
        "<=" => a <= b,
        "==" => a == b,
        "!=" => a != b,
        _ => unreachable!("Unknown conditional"),
    }
}

pub fn solve(input: &str) -> (i32, i32) {
    let mut values: HashMap<&str, i32> = HashMap::new();
    let mut max_value = i32::MIN;

    for line in input.lines() {
        // <register> <inc|dec> <amount> if <check_register> <conditional> <check_value>
        let [
            register,
            op,
            amount,
            "if",
            check_register,
            conditional,
            check_value,
        ] = line.split_whitespace().collect::<Vec<_>>()[..]
        else {
            panic!("malformed instruction: {line:?}");
        };
        let amount: i32 = amount.parse().expect("amount is an integer");
        let check_value: i32 = check_value.parse().expect("check value is an integer");

        let check_register = *values.entry(check_register).or_insert(0);
        if check_conditional(check_register, conditional, check_value) {
            let register = values.entry(register).or_insert(0);
            match op {
                "inc" => *register += amount,
                "dec" => *register -= amount,
                _ => unreachable!("not inc or dec"),
            }
            max_value = max_value.max(*register);
        }
    }

    (*values.values().max().unwrap(), max_value)
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(solve(input).0)
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(solve(input).1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
