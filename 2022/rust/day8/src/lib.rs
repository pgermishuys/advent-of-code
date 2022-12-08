#[cfg(test)]
mod tests {
    use std::{fs, vec::Vec};
    use ndarray::Array2;
    use itertools::Itertools;

    fn parse_grid(input: &str) -> Vec<Vec<u32>> {
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap())
                    .collect()
            })
            .collect()
    }

    fn get_up_down_left_right(grid: &[Vec<u32>], x: usize, y: usize) -> Vec<Vec<u32>> {
        let column = grid.iter().map(|row| row[x]).collect::<Vec<u32>>();
        let (up, down) = column.split_at(y);
        let (left, right) = grid[y].split_at(x);
        vec![
            up.iter().rev().cloned().collect(),
            down[1..].to_vec(),
            left.iter().rev().cloned().collect(),
            right[1..].to_vec(),
        ]
    }

    #[test]
    fn day8_part1() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let grid = parse_grid(contents.as_str());
        let len = grid.len();

        let answer: usize= (1..len - 1)
            .cartesian_product(1..len - 1)
            .map(|(y, x)| {
                let height = grid[y][x];
                get_up_down_left_right(&grid, x, y)
                    .iter()
                    .map(|direction| direction.iter().all(|h| *h < height))
                    .any(|direction_visible| direction_visible)
            })
            .filter(|direction_visible| *direction_visible)
            .count()
            + (len - 1) * 4;

        assert_eq!(answer, 1695);
    }

    #[test]
    fn day8_part2() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");
        
        let grid = parse_grid(contents.as_str());
        let len = grid.len();
        let answer: usize = (1..len - 1)
            .cartesian_product(1..len - 1)
            .map(|(y, x)| {
                let height = grid[y][x];
                get_up_down_left_right(&grid, x, y)
                    .iter()
                    .map(|direction| {
                        direction
                            .iter()
                            .position(|h| *h >= height)
                            .map(|p| p + 1)
                            .unwrap_or_else(|| direction.len())
                    })
                    .product()
            })
            .max()
            .unwrap();

        assert_eq!(answer, 287040);
    }
}