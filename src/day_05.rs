#[aoc(day5, part1)]
fn solve_part_1(input: &[u8]) -> usize {
    input
        .split(|&ch| ch == b'\n')
        .filter(|&line| {
            let mut vowels = 0;
            let mut has_double = false;
            let mut prev = 0_u8;
            for &ch in line {
                match ch {
                    b'a' | b'e' | b'i' | b'o' | b'u' => vowels += 1,
                    _ => {}
                }
                has_double = has_double || ch == prev;
                match (prev, ch) {
                    (b'a', b'b') | (b'c', b'd') | (b'p', b'q') | (b'x', b'y') => return false,
                    _ => {}
                }
                prev = ch;
            }
            has_double && vowels >= 3
        })
        .count()
}

#[aoc(day5, part2)]
fn solve_part_2(input: &[u8]) -> usize {
    input
        .split(|&ch| ch == b'\n')
        .filter(|&line| {
            let mut seen_pairs = [0_u32; 26];
            let mut has_pair = false;
            let mut has_single = false;
            let mut prev_prev = 0_u8;
            let mut prev = 0_u8;
            for &ch in line {
                if prev != 0 && seen_pairs[(prev - b'a') as usize] & (1 << (ch - b'a')) != 0 {
                    has_pair = true;
                }
                if prev_prev != 0 {
                    seen_pairs[(prev_prev - b'a') as usize] |= 1 << (prev - b'a');
                    if prev_prev == ch {
                        has_single = true;
                    }
                }
                (prev_prev, prev) = (prev, ch);
            }
            has_pair && has_single
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(b"ugknbfddgicrmopn" => 1)]
    #[test_case(b"aaa" => 1)]
    #[test_case(b"jchzalrnumimnmhp" => 0)]
    #[test_case(b"haegwjzuvuyypxyu" => 0)]
    #[test_case(b"dvszwmarrgswjxmb" => 0)]
    fn test_part_1(input: &[u8]) -> usize {
        solve_part_1(input)
    }

    #[test_case(b"qjhvhtzxzqqjkmpb" => 1)]
    #[test_case(b"xxyxx" => 1)]
    #[test_case(b"uurcxstgmygtbstg" => 0)]
    #[test_case(b"ieodomkazucvgmuy" => 0)]
    fn test_part_2(input: &[u8]) -> usize {
        solve_part_2(input)
    }
}
