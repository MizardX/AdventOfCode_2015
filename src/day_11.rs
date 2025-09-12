#[aoc(day11, part1)]
fn part1(input: &[u8]) -> String {
    let mut password = input.to_vec();
    prepare(&mut password);
    while !validate(&password) {
        increment(&mut password);
    }
    unsafe { String::from_utf8_unchecked(password) }
}

#[aoc(day11, part2)]
fn part2(input: &[u8]) -> String {
    let mut password = input.to_vec();
    prepare(&mut password);
    while !validate(&password) {
        increment(&mut password);
    }
    increment(&mut password);
    while !validate(&password) {
        increment(&mut password);
    }
    unsafe { String::from_utf8_unchecked(password) }
}

fn prepare(password: &mut [u8]) {
    for i in 0..password.len() {
        if matches!(password[i], b'i' | b'o' | b'l') {
            password[i] += 1;
            password[i + 1..].fill(b'a');
            break;
        }
    }
}

fn validate(password: &[u8]) -> bool {
    let mut seen_pairs = 0_u32;
    let mut count_pairs = 0;
    let mut any_straight = false;
    if password[0] == password[1] {
        seen_pairs |= 1 << (password[0] - b'a');
        count_pairs += 1;
    }
    for tri in password.windows(3) {
        let &[a, b, c] = tri else { unreachable!() };
        if b.wrapping_sub(a) == 1 && c.wrapping_sub(b) == 1 {
            any_straight = true;
        }
        if b == c {
            let bit = 1 << (b - b'a');
            if seen_pairs & bit == 0 {
                seen_pairs |= bit;
                count_pairs += 1;
            }
        }
    }
    any_straight && count_pairs >= 2
}

fn increment(password: &mut [u8]) {
    for i in (0..password.len()).rev() {
        match password[i] {
            b'z' => {
                password[i] = b'a';
            }
            b'h' | b'n' | b'k' => {
                password[i] += 2;
                return;
            }
            _ => {
                password[i] += 1;
                return;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("abcdefgh" => "abcdefgh")]
    #[test_case("abcdefgi" => "abcdefgj")]
    #[test_case("abcdofgh" => "abcdpaaa")]
    #[test_case("alcdefgh" => "amaaaaaa")]
    fn test_prepare(input: &str) -> String {
        let mut password = input.to_string().into_bytes();
        prepare(&mut password);
        unsafe { String::from_utf8_unchecked(password) }
    }

    #[test_case("hijklmmn" => false)]
    #[test_case("abbceffg" => false)]
    #[test_case("abbcegjk" => false)]
    #[test_case("abcdffaa" => true)]
    #[test_case("ghjaabcc" => true)]
    fn test_validate(input: &str) -> bool {
        validate(input.as_bytes())
    }

    #[test_case("xy" => "xz")]
    #[test_case("xz" => "ya")]
    #[test_case("zz" => "aa")]
    #[test_case("ah" => "aj")]
    #[test_case("ak" => "am")]
    #[test_case("an" => "ap")]
    fn test_increment(input: &str) -> String {
        let mut password = input.to_string().into_bytes();
        increment(&mut password);
        unsafe { String::from_utf8_unchecked(password) }
    }

    #[test_case("abcdefgh" => "abcdffaa")]
    #[test_case("ghijklmn" => "ghjaabcc")]
    fn test_part_1(input: &str) -> String {
        part1(input.as_bytes())
    }
}
