#[derive(Debug, Clone, Copy)]
enum State {
    Normal,
    Escaped,
    Hex0,
    Hex1,
}

#[aoc(day8, part1)]
fn part1(input: &[u8]) -> usize {
    let mut len = 0;
    for line in input.split(|&ch| ch == b'\n') {
        let mut state = State::Normal;
        len += line.len();
        for &ch in &line[1..line.len() - 1] {
            state = match (state, ch) {
                (State::Normal, b'\\') => State::Escaped,
                (State::Escaped, b'x' | b'X') => State::Hex0,
                (State::Hex0, _) => State::Hex1,
                _ => {
                    len -= 1;
                    State::Normal
                }
            };
        }
    }
    len
}

#[aoc(day8, part2)]
fn part2(input: &[u8]) -> usize {
    let mut len = 0;
    for line in input.split(|&ch| ch == b'\n') {
        for &ch in line {
            len += match ch {
                b'\\' | b'"' => 1,
                _ => 0,
            };
        }
        len += 2;
    }
    len
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(br#""""# => 2)]
    #[test_case(br#""abc""# => 2)]
    #[test_case(br#""abc\"abc""# => 3)]
    #[test_case(br#""\x27""# => 5)]
    fn test_part_1(input: &[u8]) -> usize {
        part1(input)
    }

    #[test_case(br#""""# => 4)]
    #[test_case(br#""abc""# => 4)]
    #[test_case(br#""abc\"abc""# => 6)]
    #[test_case(br#""\x27""# => 5)]
    fn test_part_2(input: &[u8]) -> usize {
        part2(input)
    }
}
