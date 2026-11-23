use hashbrown::HashMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let n = str::parse::<u64>(input.trim()).expect("No number provided");
    if n <= 1 {
        return Some(0);
    }
    let mut s = (n as f64).sqrt().ceil() as u64;
    if s % 2 == 0 {
        s += 1;
    }
    let r = (s - 1) / 2;
    let side_len = 2 * r;
    let max_val = s * s;
    let offset = ((max_val - n) % side_len).abs_diff(r);
    Some(r + offset)
}

pub fn part_two(input: &str) -> Option<u64> {
    let target = str::parse::<u64>(input.trim()).expect("No number provided");
    let mut grid = HashMap::new();
    grid.insert((0i32, 0i32), 1u64);

    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let (mut x, mut y) = (0, 0);
    let mut dir_idx = 0;
    let mut step_len = 1;

    loop {
        for _ in 0..2 {
            let (dx, dy) = dirs[dir_idx];
            for _ in 0..step_len {
                x += dx;
                y += dy;

                let mut sum = 0;
                for nx in (x - 1)..=(x + 1) {
                    for ny in (y - 1)..=(y + 1) {
                        if (nx, ny) != (x, y) {
                            sum += grid.get(&(nx, ny)).copied().unwrap_or(0);
                        }
                    }
                }

                if sum > target {
                    return Some(sum);
                }
                grid.insert((x, y), sum);
            }
            dir_idx = (dir_idx + 1) % 4;
        }
        step_len += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("1"), Some(0));
        assert_eq!(part_one("12"), Some(3));
        assert_eq!(part_one("23"), Some(2));
        assert_eq!(part_one("1024"), Some(31));
    }

    #[test]
    fn test_part_two() {
        assert!(true)
    }
}
