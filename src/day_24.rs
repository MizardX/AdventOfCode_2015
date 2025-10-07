use std::num::ParseIntError;

#[aoc_generator(day24)]
fn parse(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day24, part1)]
fn part_1(packages: &[u32]) -> u64 {
    let sum: u32 = packages.iter().copied().sum();
    let target = sum / 3;

    let mut target_combinations = find_combinations_by_weight(packages, target);
    target_combinations.sort_unstable_by_key(|&m| m.count_ones());

    let mut minimum = (u32::MAX, u64::MAX); // (size, QE)
    for &combination1 in &target_combinations {
        let size1 = combination1.count_ones();
        if size1 > minimum.0 {
            break;
        }
        for &combination2 in &target_combinations {
            if combination1 & combination2 != 0 {
                continue;
            }
            // We have two non-overlapping combinations with 1/3 weight each. The third
            // combination is the remaining packages, which must also have 1/3 weight.
            //
            // The QE is the product of the packages in `combination1`. Once we know
            // there are at least one `combination2`, we do not need to look for more,
            // and can skip to the next `combination1`.
            let quantum_entanglement = packages
                .iter()
                .enumerate()
                .filter_map(|(j, &package)| {
                    ((combination1 & (1 << j)) != 0).then_some(u64::from(package))
                })
                .product();
            minimum = minimum.min((size1, quantum_entanglement));
            break;
        }
    }
    minimum.1
}

#[aoc(day24, part2)]
fn part_2(packages: &[u32]) -> u64 {
    let sum: u32 = packages.iter().copied().sum();
    let target = sum / 4;

    let mut target_combinations = find_combinations_by_weight(packages, target);
    target_combinations.sort_unstable_by_key(|&m| m.count_ones());

    let mut minimum = (u32::MAX, u64::MAX); // (size, QE)

    'next_combination1: for &combination1 in &target_combinations {
        let size = combination1.count_ones();
        if size > minimum.0 {
            break;
        }
        for &combination2 in &target_combinations {
            if combination1 & combination2 != 0 {
                continue;
            }
            for &combination3 in &target_combinations {
                if (combination1 | combination2) & combination3 != 0 {
                    continue;
                }
                // We have three non-overlapping combinations with 1/4 weight each. The fourth
                // combination is the remaining packages, which must also have 1/4 weight.
                // The QE is the product of the packages in `combination1`. Once we know
                // there are at least one `combination2` and `combination3`, we do not need
                // to look for more, and can skip to the next `combination1`.
                let quantum_entanglement = packages
                    .iter()
                    .enumerate()
                    .filter_map(|(package_index, &package)| {
                        ((combination1 & (1 << package_index)) != 0).then_some(u64::from(package))
                    })
                    .product();
                minimum = minimum.min((size, quantum_entanglement));
                continue 'next_combination1;
            }
        }
    }
    minimum.1
}

fn find_combinations_by_weight(packages: &[u32], target: u32) -> Vec<u32> {
    let mut combinations_by_weight = vec![vec![]; target as usize + 1];
    combinations_by_weight[0].push(0);

    // Try to add each package to previous combiantions. Since it is a new package, we
    // know it is not already included.
    for (package_index, &package) in packages.iter().enumerate() {
        let package_bit = 1u32 << package_index;
        for total in (package..=target).rev() {
            // Temporarily take out the Vec, so we can modify it while iterating over the other combinations.
            let mut combinations_total =
                std::mem::take(&mut combinations_by_weight[total as usize]);
            for &combination in &combinations_by_weight[(total - package) as usize] {
                combinations_total.push(combination | package_bit);
            }
            combinations_by_weight[total as usize] = combinations_total;
        }
    }

    combinations_by_weight.swap_remove(target as usize)
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
