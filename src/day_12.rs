use std::collections::HashMap;

#[aoc(day12, part1)]
fn part_1(input: &[u8]) -> i64 {
    let mut sum = 0;
    lex(input, |token| {
        if let Token::Integer(num) = token {
            sum += num;
        }
    });
    sum
}

#[aoc(day12, part2)]
fn part_2(input: &[u8]) -> i64 {
    let mut context = Vec::new();
    let mut current = Context::Array;
    let mut sum = 0;
    lex(input, |token| match token {
        Token::Integer(val) => {
            if let Context::Object(ref mut octx) = current {
                octx.expects_value = !octx.expects_value;
            }
            sum += val;
        }
        Token::String(s) => {
            if let Context::Object(ref mut octx) = current {
                if octx.expects_value && s == "red" {
                    octx.rollback_sum = true;
                }
                octx.expects_value = !octx.expects_value;
            }
        }
        Token::ArrayStart => {
            context.push(current);
            current = Context::Array;
        }
        Token::ArrayEnd => {
            current = context.pop().unwrap();
            if let Context::Object(ref mut octx) = current {
                octx.expects_value = !octx.expects_value;
            }
        }
        Token::ObjectStart => {
            context.push(current);
            current = Context::Object(ObjectContext {
                sum_before: sum,
                rollback_sum: false,
                expects_value: false,
            });
        }
        Token::ObjectEnd => {
            if let Context::Object(octx) = current
                && octx.rollback_sum
            {
                sum = octx.sum_before;
            }
            current = context.pop().unwrap();
            if let Context::Object(ref mut octx) = current {
                octx.expects_value = !octx.expects_value;
            }
        }
    });
    assert_eq!(&context, &[]);
    sum
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Context {
    Array,
    Object(ObjectContext),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ObjectContext {
    sum_before: i64,
    rollback_sum: bool,
    expects_value: bool,
}

#[derive(Debug, Clone)]
enum State {
    Initial,
    Integer(i64),
    Negative(i64),
    String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(unused)]
enum Token<'a> {
    Integer(i64),
    String(&'a str),
    ArrayStart,
    ArrayEnd,
    ObjectStart,
    ObjectEnd,
}

fn lex<F>(input: &[u8], mut callback: F)
where
    F: FnMut(Token<'_>),
{
    let mut stringvalues = HashMap::<Vec<u8>, String>::new();
    let mut state = State::Initial;
    let mut str_val = Vec::new();
    for &ch in input {
        let mut again = true;
        while again {
            again = false;
            state = match (state, ch) {
                (State::Initial, b'-') => State::Negative(0),
                (State::Initial, b'0'..=b'9') => State::Integer(i64::from(ch - b'0')),
                (State::Initial, b'[') => {
                    callback(Token::ArrayStart);
                    State::Initial
                }
                (State::Initial, b']') => {
                    callback(Token::ArrayEnd);
                    State::Initial
                }
                (State::Initial, b'{') => {
                    callback(Token::ObjectStart);
                    State::Initial
                }
                (State::Initial, b'}') => {
                    callback(Token::ObjectEnd);
                    State::Initial
                }
                (State::Initial, b'"') => State::String,
                (State::Initial, _) => State::Initial,
                (State::Integer(val), b'0'..=b'9') => {
                    State::Integer(val * 10 + i64::from(ch - b'0'))
                }
                (State::Negative(val), b'0'..=b'9') => {
                    State::Negative(val * 10 - i64::from(ch - b'0'))
                }
                (State::Integer(val) | State::Negative(val), _) => {
                    callback(Token::Integer(val));
                    again = true;
                    State::Initial
                }
                (State::String, b'"') => {
                    let str_val_copy = str_val.split_off(0);
                    let entry = stringvalues
                        .entry(str_val_copy)
                        .or_insert_with_key(|b| unsafe { String::from_utf8_unchecked(b.clone()) });
                    callback(Token::String(&*entry));
                    State::Initial
                }
                (State::String, _) => {
                    str_val.push(ch);
                    State::String
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(br"[1,2,3]", &[Token::ArrayStart, Token::Integer(1), Token::Integer(2), Token::Integer(3), Token::ArrayEnd])]
    #[test_case(br"[-123]", &[Token::ArrayStart, Token::Integer(-123), Token::ArrayEnd])]
    #[test_case(br#"[1,{"c":"red","b":2},3]"#, &[Token::ArrayStart, Token::Integer(1), Token::ObjectStart, Token::String("c"), Token::String("red"), Token::String("b"),Token::Integer(2), Token::ObjectEnd, Token::Integer(3), Token::ArrayEnd])]
    fn test_lex(input: &[u8], expects: &[Token]) {
        let mut it = expects.iter();
        lex(input, |tok| {
            assert_eq!(&tok, it.next().unwrap());
        });
        assert!(it.next().is_none());
    }

    #[test_case(br"[1,2,3]" => 6)]
    #[test_case(br#"{"a":2,"b":4}"# => 6)]
    #[test_case(br"[[[3]]]" => 3)]
    #[test_case(br#"{"a":{"b":4},"c":-1}"# => 3)]
    #[test_case(br#"{"a":[-1,1]}"# => 0)]
    #[test_case(br#"[-1,{"a":1}]"# => 0)]
    #[test_case(br"[]" => 0; "empty arr expects 0")]
    #[test_case(br"{}" => 0; "empty obj expects 0")]
    fn test_part_1(input: &[u8]) -> i64 {
        part_1(input)
    }

    #[test_case(br"[1,2,3]" => 6)]
    #[test_case(br#"[1,{"c":"red","b":2},3]"# => 4)]
    #[test_case(br#"{"d":"red","e":[1,2,3,4],"f":5}"# => 0)]
    #[test_case(br#"[1,"red",5]"# => 6)]
    fn test_part_2(input: &[u8]) -> i64 {
        part_2(input)
    }
}
