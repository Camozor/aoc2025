use std::fs::read_to_string;

fn main() {
    let lines = read_lines("input.txt");
    let mut grid = Grid::from_lines(lines);

    println!("Response v1={}", grid.v1());
    println!("Response v2={}", grid.v2());
}

fn read_lines(file: &str) -> Vec<String> {
    read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn from_lines(lines: Vec<String>) -> Self {
        let mut grid = vec![];
        for line in lines {
            let grid_line: Vec<char> = line.chars().collect();
            grid.push(grid_line);
        }
        Grid { grid }
    }

    fn v1(&self) -> i32 {
        let mut counter = 0;

        for x in 0..self.grid.len() {
            for y in 0..self.grid[0].len() {
                if self.is_roll(x, y) {
                    let neighbours = self.count_neighbours(x as i32, y as i32);
                    if neighbours < 4 {
                        counter += 1;
                    }
                }
            }
        }

        counter
    }

    fn v2(&mut self) -> i32 {
        let mut counter = 0;

        let mut has_at_least_one_change = true;

        while has_at_least_one_change {
            has_at_least_one_change = false;

            for x in 0..self.grid.len() {
                for y in 0..self.grid[0].len() {
                    if self.is_roll(x, y) {
                        let neighbours = self.count_neighbours(x as i32, y as i32);
                        if neighbours < 4 {
                            counter += 1;
                            self.mark_as_removed(x, y);
                            has_at_least_one_change = true;
                        }
                    }
                }
            }
        }

        counter
    }

    fn count_neighbours(&self, x: i32, y: i32) -> i32 {
        let mut neighbours = 0;
        neighbours += self.add_neighbour(x - 1, y - 1);
        neighbours += self.add_neighbour(x - 1, y);
        neighbours += self.add_neighbour(x - 1, y + 1);
        neighbours += self.add_neighbour(x, y - 1);
        neighbours += self.add_neighbour(x, y + 1);
        neighbours += self.add_neighbour(x + 1, y - 1);
        neighbours += self.add_neighbour(x + 1, y);
        neighbours += self.add_neighbour(x + 1, y + 1);

        neighbours
    }

    fn add_neighbour(&self, x: i32, y: i32) -> i32 {
        if self.is_neighbour(x, y) {
            return 1;
        }
        0
    }

    fn is_neighbour(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            return false;
        }

        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();

        x < self.grid.len().try_into().unwrap()
            && y < self.grid[0].len().try_into().unwrap()
            && self.is_roll(x, y)
    }

    fn is_roll(&self, x: usize, y: usize) -> bool {
        self.grid[x][y] == '@'
    }

    fn mark_as_removed(&mut self, x: usize, y: usize) {
        self.grid[x][y] = 'x';
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_neighbour() {
        let lines = vec![String::from("..@@.@@@@."), String::from("@@@.@.@.@@")];
        let grid = Grid::from_lines(lines);
        assert_eq!(grid.count_neighbours(0, 2), 3);
    }
}
