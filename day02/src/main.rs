use std::{fs::read_to_string, str::FromStr};

fn main() {
    let response_v1 = v1("input.txt");
    println!("Response v1={response_v1}");

    let response_v2 = v2("input.txt");
    println!("Response v2={response_v2}");
}

#[derive(Debug)]
struct Range {
    first: String,
    last: String,
}

fn parse_ranges(line: String) -> Vec<Range> {
    let mut computed_ranges: Vec<Range> = vec![];
    let ranges: Vec<&str> = line.split(',').collect();
    for range in ranges {
        let range: Vec<&str> = range.split('-').collect();
        let first = String::from_str(range.first().unwrap()).unwrap();
        let last = String::from_str(range.last().unwrap())
            .unwrap()
            .replace('\n', "");

        computed_ranges.push(Range { first, last });
    }

    computed_ranges
}

fn is_id_valid(id: &str) -> bool {
    if id.len() % 2 != 0 {
        return true;
    }

    let (first_half, second_half) = id.split_at(id.len() / 2);
    first_half != second_half
}

fn is_id_valid_v2(id: &str) -> bool {
    if id.len() < 2 {
        return true;
    }

    for substring_len in 1..(id.len() / 2 + 1) {
        let mut old_substring = &id[0..substring_len];

        let mut substring_index = substring_len;
        while substring_index <= id.len() - substring_len {
            let max_index = std::cmp::min(substring_index + substring_len, id.len());
            let substring = &id[substring_index..max_index];

            if old_substring != substring {
                break;
            }

            old_substring = substring;

            if substring_index == id.len() - substring_len {
                return false;
            }

            substring_index += substring_len;
        }
    }

    true
}

fn find_all_invalids(range: &Range) -> Vec<u64> {
    let start: u64 = range.first.parse().unwrap();
    let end: u64 = range.last.parse().unwrap();

    let mut invalids = vec![];

    for i in start..end + 1 {
        if !is_id_valid(i.to_string().as_str()) {
            invalids.push(i);
        }
    }

    invalids
}

fn find_all_invalids_v2(range: &Range) -> Vec<u64> {
    let start: u64 = range.first.parse().unwrap();
    let end: u64 = range.last.parse().unwrap();

    let mut invalids = vec![];

    for i in start..end + 1 {
        if !is_id_valid_v2(i.to_string().as_str()) {
            invalids.push(i);
        }
    }

    invalids
}

fn v1(file_name: &str) -> u64 {
    let line = read_to_string(file_name).unwrap();
    let ranges = parse_ranges(line);

    let mut counter = 0;
    for range in ranges {
        let invalids = find_all_invalids(&range);
        let sum: u64 = invalids.iter().sum();
        counter += sum;
    }

    counter
}

fn v2(file_name: &str) -> u64 {
    let line = read_to_string(file_name).unwrap();
    let ranges = parse_ranges(line);

    let mut counter = 0;
    for range in ranges {
        let invalids = find_all_invalids_v2(&range);
        let sum: u64 = invalids.iter().sum();
        counter += sum;
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ranges() {
        let line = String::from("11-22,95-115");
        let ranges = parse_ranges(line);
        assert_eq!(ranges.len(), 2);
        let first_range = ranges.first().unwrap();
        assert_eq!(first_range.first, String::from("11"));
        assert_eq!(first_range.last, String::from("22"));

        let second_range = ranges.iter().nth(1).unwrap();
        assert_eq!(second_range.first, String::from("95"));
        assert_eq!(second_range.last, String::from("115"));
    }

    #[test]
    fn test_is_id_valid() {
        assert_eq!(is_id_valid("7"), true);
        assert_eq!(is_id_valid("76854"), true);
        assert_eq!(is_id_valid("12"), true);
        assert_eq!(is_id_valid("77"), false);
        assert_eq!(is_id_valid("38593859"), false);
    }

    #[test]
    fn test_find_all_invalids() {
        let r = Range {
            first: String::from("11"),
            last: String::from("22"),
        };
        let invalids = find_all_invalids(&r);
        assert_eq!(invalids.len(), 2);
        assert_eq!(invalids.first().unwrap(), &11);
        assert_eq!(invalids.last().unwrap(), &22);
    }

    #[test]
    fn test_is_id_valid_v2() {
        assert_eq!(is_id_valid_v2("7"), true);
        assert_eq!(is_id_valid_v2("77"), false);
        assert_eq!(is_id_valid_v2("777"), false);
        assert_eq!(is_id_valid_v2("1212121212"), false);
        assert_eq!(is_id_valid_v2("824824824"), false);
        assert_eq!(is_id_valid_v2("824824825"), true);
    }
}
