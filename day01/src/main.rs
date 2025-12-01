use std::{fs::read_to_string, path::absolute};

fn main() {
    let lines = read_lines("input.txt");
    let lines = lines.iter().map(AsRef::as_ref).collect();
    let password = compute_password(lines);
    println!("Password={password}");
}

fn compute_password(lines: Vec<&str>) -> i32 {
    let mut counter = 0;
    let mut current_position = 50;

    for line in lines {
        current_position += build_move(line);
        current_position = current_position % 100;
        if current_position == 0 {
            counter += 1;
        }
    }

    counter
}

fn compute_password_v2(lines: Vec<&str>) -> i32 {
    let mut counter = 0;
    let mut current_position = 50;

    for line in lines {
        let previous_position = current_position;
        let the_move = build_move(line);
        current_position += the_move;
        current_position = current_position % 100;

        if the_move < 0 && current_position > previous_position {
            println!("Increment on the left {line}");
            counter += 1;
        }
        if the_move > 0 && current_position < previous_position {
            println!("Increment on the right {line}");
            counter += 1;
        }
        if current_position == 0 {
            println!("Increment at {line}");
            counter += 1;
        }
    }

    counter
}

struct MovementInput {
    position: i32,
    movement: i32,
}

struct MovementOutput {
    position: i32,
    password_increment: i32,
}

fn compute_move(input: MovementInput) -> MovementOutput {
    let mut counter = 0;

    let mut position = input.position;
    let mut movement = input.movement;

    while movement.abs() > 100 {
        if movement < 0 {
            movement = movement + 100;
        } else {
            movement = movement - 100;
        }
        counter += 1;
    }
    position = (position + movement) % 100;

    if position == 0 {
        counter += 1;
    }

    MovementOutput {
        position: position,
        password_increment: counter,
    }
}

fn build_move(line: &str) -> i32 {
    let orientation = line.chars().nth(0).unwrap_or('R');
    let orientation = match orientation {
        'R' => 1,
        'L' => -1,
        _ => 1,
    };

    let (_, number_str) = line.split_at(1);
    let n: i32 = number_str.parse().expect("failed to parse");

    orientation * n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_password() {
        let lines = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];
        assert_eq!(compute_password(lines), 3);
    }

    #[test]
    fn test_compute_move_output_simple() {
        let input = MovementInput {
            position: 99,
            movement: 1,
        };
        let output = compute_move(input);
        assert_eq!(output.position, 0);
        assert_eq!(output.password_increment, 1);
    }

    #[test]
    fn test_compute_move_output_overflow() {
        let input = MovementInput {
            position: 99,
            movement: 2,
        };
        let output = compute_move(input);
        assert_eq!(output.position, 1);
        assert_eq!(output.password_increment, 1);
    }

    // #[test]
    // fn test_compute_move_output_multiple_spins() {
    //     let input = MovementInput {
    //         position: 99,
    //         movement: 302,
    //     };
    //     let output = compute_move(input);
    //     assert_eq!(output.position, 1);
    //     assert_eq!(output.password_increment, 3);
    // }

    // #[test]
    // fn test_compute_password_v2() {
    //     let lines = vec![
    //         "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    //     ];
    //     assert_eq!(compute_password_v2(lines), 6);
    // }

    #[test]
    fn test_compute_move() {
        assert_eq!(build_move("L32"), -32);
        assert_eq!(build_move("R678"), 678);
    }
}

fn read_lines(file: &str) -> Vec<String> {
    read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
