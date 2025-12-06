use std::fs::read_to_string;

fn main() {
    println!("v1 response={}", v1("input.txt"));
}

fn v1(path_name: &str) -> u32 {
    let banks = read_lines(path_name);

    let mut counter = 0;
    for bank in banks {
        counter += max_joltage(&bank);
    }

    counter
}

fn max_joltage(bank: &str) -> u32 {
    let mut index_first_digit = 0;
    let mut first_digit = get_nth(bank, 0);

    for index in 1..bank.len() - 1 {
        let current = get_nth(bank, index);
        if current > first_digit {
            index_first_digit = index;
            first_digit = current;
        }
    }

    let start_index_last_digit = index_first_digit + 1;
    let mut last_digit = get_nth(bank, start_index_last_digit);

    for index in start_index_last_digit + 1..bank.len() {
        let current = get_nth(bank, index);
        if current > last_digit {
            last_digit = current;
        }
    }

    10 * first_digit + last_digit
}

fn get_nth(bank: &str, n: usize) -> u32 {
    bank.chars().nth(n).unwrap().to_string().parse().unwrap()
}

fn read_lines(file: &str) -> Vec<String> {
    read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_joltage() {
        assert_eq!(max_joltage("987654321111111"), 98);
        assert_eq!(max_joltage("811111111111119"), 89);
        assert_eq!(max_joltage("234234234234278"), 78);
        assert_eq!(max_joltage("818181911112111"), 92);
    }
}
