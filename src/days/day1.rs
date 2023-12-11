fn do_line(s: &str) -> i32 {
    let mut first_char: Option<String> = None;
    let mut last_char: Option<String> = None;

    for c in s.chars() {
        let val = c.to_digit(10);
        match val {
            Some(_) => {
                if first_char.is_none() {
                    first_char = Some(c.to_string());
                }
                last_char = Some(c.to_string());
            },
            None => (),
        }
    }

    return format!("{}{}",first_char.unwrap(), last_char.unwrap()).parse().unwrap()
}

fn do_line_2(s: &str) -> i32 {
    let words: Vec<(&str, u32)> = vec!(
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    );

    let mut first_num: Option<u32> = None;
    let mut last_num: Option<u32> = None;

    let mut i = 0;
    let mut j = 0;
    let bytes = s.as_bytes();
    while i < s.len() {
        let c = bytes[i] as char;
        match c {
            '1' | '2' | '3' | '4' | '5' | '6'| '7' | '8' |'9' | '0' => {
                if first_num.is_none() {
                    first_num = Some(c.to_digit(10).unwrap());
                }
                last_num = Some(c.to_digit(10).unwrap());
                i = i +1;
            }
            _ => {
                if j <= i {
                    j = i + 1;
                }
                while j < s.len() + 1 {
                    let check_str = &s[i..j];
                    let possible_words: Vec<&(&str, u32)> = words.iter().filter(|&(k, _)| k.starts_with(check_str)).collect();
                    match possible_words.len() {
                        0 => {
                            break;
                        },
                        1 => {
                            if possible_words[0].0 == check_str {
                                if first_num.is_none() {
                                    first_num = Some(possible_words[0].1);
                                }
                                last_num = Some(possible_words[0].1);
                                break;
                            }
                            j = j + 1;
                        },
                        _ => {
                            j = j +1;
                        }
                    }
                }
                i = i + 1;

            }
        }
    }
    return format!("{}{}",first_num.unwrap(), last_num.unwrap()).parse().unwrap()
}

pub fn part1(input: &String) -> i32 {
    let mut running_total = 0;

    for line in input.lines() {
        running_total += do_line(line)
    }

    return running_total
}

pub fn part2(input: &String) -> i32 {
    let mut running_total = 0;
    for line in input.lines() {
        running_total += do_line_2(line)
    }

    return running_total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = String::from("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");

        assert_eq!(part1(&input), 142)

    }

    #[test]
    fn test_part2() {
        let input = String::from("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
    assert_eq!(part2(&input), 281)

    }
}