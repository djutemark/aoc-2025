const SAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn main() {
    let id_ranges = IdRange::parse_multiple(SAMPLE_INPUT);

    let all_invalids = id_ranges
        .iter()
        .flat_map(|id_range| id_range.iter().find_invalids())
        .collect::<Vec<String>>();

    let sum_of_invalids = all_invalids
        .iter()
        .map(|i| i.parse::<u64>().unwrap())
        .sum::<u64>();

    println!("The sum is: {}", sum_of_invalids);
}

#[derive(PartialEq, Debug)]
struct IdRange<'a> {
    start: &'a str,
    end: &'a str,
}

struct IdRangeIterator<'a> {
    end: &'a str,
    current: usize,
}

impl<'a> Iterator for IdRangeIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let end: usize = self.end.parse().unwrap();

        if self.current > end {
            return None;
        }

        let value = self.current.to_string();

        self.current += 1;

        Some(value)
    }
}

impl<'a> IdRangeIterator<'a> {
    fn find_invalids(self) -> Vec<String> {
        let mut invalids: Vec<String> = Vec::new();

        for id in self.collect::<Vec<String>>() {
            if is_invalid(&id) {
                invalids.push(id);
            }
        }

        invalids
    }
}

impl<'a> IdRange<'a> {
    fn parse(input: &'a str) -> Self {
        let split: Vec<&str> = input.split("-").collect();

        let start = split[0];
        let end = split[1];

        Self { start, end }
    }

    fn iter(&self) -> IdRangeIterator<'a> {
        let current: usize = self.start.parse().unwrap();

        IdRangeIterator {
            end: self.end,
            current,
        }
    }

    fn parse_multiple(comma_separated_input: &'a str) -> Vec<Self> {
        let splitted = comma_splitter(comma_separated_input);

        splitted.iter().map(|s| IdRange::parse(s)).collect()
    }
}

fn comma_splitter(sample_input_short: &str) -> Vec<&str> {
    sample_input_short.split(",").collect()
}

fn is_invalid(s: &str) -> bool {
    let length = s.len();

    let half_length = length / 2; // We can never have two equal parts where one is longer than the other

    for pattern_length in 1..=half_length {
        // We must parts with all equal length
        if length % pattern_length != 0 {
            continue;
        }

        let (pattern, _) = s.split_at(pattern_length);

        let num_patterns_fit = s.len() / pattern_length;

        let mut ok = false;

        for i_split in 0..num_patterns_fit {
            let start = i_split * pattern_length;
            let end = start + pattern_length;

            let part = &s[start..end];

            ok = part == pattern;

            if !ok {
                break;
            }
        }

        if ok {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{IdRange, comma_splitter, is_invalid};

    const SAMPLE_INPUT_SINGLE: &str = "11-22";
    const SAMPLE_INPUT_SHORT: &str = "11-22,95-115,998-1012";

    #[test]
    fn test_id_range_parses_correctly() {
        let id_range = IdRange::parse(SAMPLE_INPUT_SINGLE);

        assert_eq!(
            id_range,
            IdRange {
                start: "11",
                end: "22"
            }
        );
    }

    #[test]
    fn test_comma_splitter() {
        let result = comma_splitter(SAMPLE_INPUT_SHORT);

        assert_eq!(result, vec!["11-22", "95-115", "998-1012"]);
    }

    #[test]
    fn test_parse_multiple() {
        let id_ranges = IdRange::parse_multiple(SAMPLE_INPUT_SHORT);

        assert_eq!(
            id_ranges,
            vec![
                IdRange {
                    start: "11",
                    end: "22"
                },
                IdRange {
                    start: "95",
                    end: "115"
                },
                IdRange {
                    start: "998",
                    end: "1012"
                },
            ]
        );
    }

    #[test]
    fn test_is_invalid() {
        assert_eq!(is_invalid("101"), false);
        assert_eq!(is_invalid("1234"), false);
        assert_eq!(is_invalid("38593862"), false);

        assert_eq!(is_invalid("11"), true);
        assert_eq!(is_invalid("22"), true);
        assert_eq!(is_invalid("1188511885"), true);
        assert_eq!(is_invalid("446446"), true);
        assert_eq!(is_invalid("38593859"), true);

        assert_eq!(is_invalid("12341234"), true);
        assert_eq!(is_invalid("123123123"), true);
        assert_eq!(is_invalid("1212121212"), true);
        assert_eq!(is_invalid("1111111"), true);
    }

    #[test]
    fn test_id_range_iterators() {
        let id_range = IdRange {
            start: "9",
            end: "12",
        };

        assert_eq!(
            id_range.iter().collect::<Vec<String>>(),
            vec![
                String::from("9"),
                String::from("10"),
                String::from("11"),
                String::from("12")
            ]
        );
    }

    #[test]
    fn test_id_range_iterator_invalids() {
        assert_eq!(
            IdRange::parse("11-22").iter().find_invalids(),
            vec![String::from("11"), String::from("22")]
        );
        assert_eq!(
            IdRange::parse("95-115").iter().find_invalids(),
            vec![String::from("99"), String::from("111"),]
        );
        assert_eq!(
            IdRange::parse("998-1012").iter().find_invalids(),
            vec![String::from("999"), String::from("1010"),]
        );
        assert_eq!(
            IdRange::parse("1188511880-1188511890")
                .iter()
                .find_invalids(),
            vec![String::from("1188511885"),]
        );
        assert_eq!(
            IdRange::parse("222220-222224").iter().find_invalids(),
            vec![String::from("222222"),]
        );
        assert_eq!(
            IdRange::parse("1698522-1698528").iter().find_invalids(),
            Vec::<String>::new(),
        );
        assert_eq!(
            IdRange::parse("446443-446449").iter().find_invalids(),
            vec![String::from("446446"),]
        );
        assert_eq!(
            IdRange::parse("38593856-38593862").iter().find_invalids(),
            vec![String::from("38593859"),]
        );
        assert_eq!(
            IdRange::parse("565653-565659").iter().find_invalids(),
            vec![String::from("565656"),]
        );
        assert_eq!(
            IdRange::parse("824824821-824824827").iter().find_invalids(),
            vec![String::from("824824824"),]
        );
        assert_eq!(
            IdRange::parse("2121212118-2121212124")
                .iter()
                .find_invalids(),
            vec![String::from("2121212121"),]
        );
    }
}
