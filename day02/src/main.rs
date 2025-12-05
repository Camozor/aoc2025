use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

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
        let last = String::from_str(range.last().unwrap()).unwrap();

        computed_ranges.push(Range { first, last });
    }

    computed_ranges
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
}
