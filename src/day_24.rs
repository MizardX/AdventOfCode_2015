use std::num::ParseIntError;

#[aoc_generator(day24)]
fn parse(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day24, part1)]
fn part_1(input: &[u32]) -> u64 {
    let sum: u32 = input.iter().copied().sum();
    let target = sum / 3;
    let mut groups = vec![vec![]; target as usize + 1];
    groups[0].push(0);
    for (i, &x) in input.iter().enumerate() {
        for y in (x..=target).rev() {
            let mut g = std::mem::take(&mut groups[y as usize]);
            for mask in &groups[(y - x) as usize] {
                g.push(mask | (1u32 << i));
            }
            groups[y as usize] = g;
        }
    }
    let mut min = (u32::MAX, u64::MAX);
    groups[target as usize].sort_by_key(|&m| m.count_ones());
    let target_group = groups[target as usize].as_slice();
    for &mask1 in target_group {
        let size1 = mask1.count_ones();
        if size1 > min.0 {
            continue;
        }
        for &mask2 in target_group {
            if mask1 & mask2 != 0 {
                continue;
            }
            let qe = input
                .iter()
                .enumerate()
                .filter_map(|(j, &y)| ((mask1 & (1 << j)) != 0).then_some(u64::from(y)))
                .product();
            min = min.min((size1, qe));
            break;
        }
    }
    min.1
}

#[aoc(day24, part2)]
fn part_2(input: &[u32]) -> u64 {
    let sum: u32 = input.iter().copied().sum();
    let target = sum / 4;
    let mut groups = vec![vec![]; target as usize + 1];
    groups[0].push(0);
    for (i, &x) in input.iter().enumerate() {
        for y in (x..=target).rev() {
            let mut g = std::mem::take(&mut groups[y as usize]);
            for mask in &groups[(y - x) as usize] {
                g.push(mask | (1u32 << i));
            }
            groups[y as usize] = g;
        }
    }
    let mut min = (u32::MAX, u64::MAX);
    groups[target as usize].sort_by_key(|&m| m.count_ones());
    let target_group = groups[target as usize].as_slice();
    'mask1: for &mask1 in target_group {
        let size = mask1.count_ones();
        if size > min.0 {
            continue;
        }
        for &mask2 in target_group {
            if mask1 & mask2 != 0 {
                continue;
            }
            for &mask3 in target_group {
                if (mask1 | mask2) & mask3 != 0 {
                    continue;
                }
                let qe = input
                    .iter()
                    .enumerate()
                    .filter_map(|(j, &y)| ((mask1 & (1 << j)) != 0).then_some(u64::from(y)))
                    .product();
                min = min.min((size, qe));
                continue 'mask1;
            }
        }
    }
    min.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let nums = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let result = part_1(&nums);
        assert_eq!(result, 99);
    }

    #[test]
    fn test_part_2() {
        let nums = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let result = part_2(&nums);
        assert_eq!(result, 44);
    }
}
