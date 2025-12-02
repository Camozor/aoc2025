use std::fs::read_to_string;

fn main() {
    let lines = read_lines("input.txt");
    let lines = lines.iter().map(AsRef::as_ref).collect();
    // let password = compute_password(lines);
    // println!("Password={password}");

    let password_v2 = compute_password_v2(lines);
    println!("Password_v2={password_v2}");
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
        let output = compute_move(MovementInput {
            position: current_position,
            movement: build_move(line),
        });
        current_position = output.position;
        counter += output.password_increment;

        println!("End of {line} {current_position} {counter}");
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

    let spins = input.movement / 100;
    counter += spins.abs();

    let mut new_position = input.position;
    let movement = input.movement - (spins * 100);

    let old_position = new_position;

    new_position = (new_position + movement).rem_euclid(100);

    if new_position == 0 {
        counter += 1;
    } else {
        if input.position != 0
            && ((movement > 0 && new_position < old_position)
                || (movement < 0 && new_position > old_position))
        {
            counter += 1;
        }
    }

    MovementOutput {
        position: new_position,
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

    #[test]
    fn test_compute_move_output_multiple_spins() {
        let input = MovementInput {
            position: 99,
            movement: 302,
        };
        let output = compute_move(input);
        assert_eq!(output.position, 1);
        assert_eq!(output.password_increment, 4);
    }

    #[test]
    fn test_compute_move_output_backwards() {
        let input = MovementInput {
            position: 99,
            movement: -125,
        };
        let output = compute_move(input);
        assert_eq!(output.position, 74);
        assert_eq!(output.password_increment, 1);
    }

    #[test]
    fn test_compute_password_v2() {
        let lines = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];
        assert_eq!(compute_password_v2(lines), 6);
    }

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
