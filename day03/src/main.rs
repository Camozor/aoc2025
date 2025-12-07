use std::fs::read_to_string;

fn main() {
    println!("v1 response={}", v1("input.txt"));
    println!("v2 response={}", v2("input.txt"));
}

fn v1(path_name: &str) -> u64 {
    let banks = read_lines(path_name);

    let mut counter = 0;
    for bank in banks {
        counter += max_joltage(&bank);
    }

    counter
}

fn v2(path_name: &str) -> u64 {
    let banks = read_lines(path_name);

    let mut counter = 0;
    for bank in banks {
        counter += max_joltage_v2(&bank);
    }

    counter
}

fn max_joltage(bank: &str) -> u64 {
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

fn max_joltage_v2(bank: &str) -> u64 {
    let mut digits: [u64; 12] = [0; 12];
    let mut found_index = 0;

    let mut counter = 0;
    for digit_index in 0..12 {
        let min_possible_index = found_index;
        let max_possible_index = bank.len() - 12 + digit_index;

        digits[digit_index] = 0;

        for index in min_possible_index..max_possible_index + 1 {
            let current = get_nth(bank, index);
            if current > digits[digit_index] {
                found_index = index;
                digits[digit_index] = current;
            }
        }

        counter += digits[digit_index] * (10 as u64).pow(12 - 1 - digit_index as u32);
        found_index = found_index + 1;
    }

    counter
}

fn get_nth(bank: &str, n: usize) -> u64 {
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

    #[test]
    fn test_max_joltage_v2() {
        assert_eq!(max_joltage_v2("987654321111111"), 987654321111);
        assert_eq!(max_joltage_v2("234234234234278"), 434234234278);
        assert_eq!(max_joltage_v2("818181911112111"), 888911112111);
    }
}
