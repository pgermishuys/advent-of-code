#[cfg(test)]
mod tests {
    use std::{fs, fmt, collections::HashMap};
    //from: https://github.com/Ottigan/advent_of_code/blob/main/day7/src/main.rs
    #[test]
    fn day7_part1() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let mut directories: HashMap<String, usize> = HashMap::from([(String::from("/"), 0)]);

        let mut current_dir = Vec::from(["/"]);
        for line in contents.lines().skip(2) {
            if line.starts_with("$ cd") {
                let dir = line.split_whitespace().last().unwrap();

                if dir.ne("..") {
                    current_dir.push(dir);
                    let current_dir_str = current_dir.join("");
                    directories.insert(current_dir_str, 0);
                } else {
                    current_dir.pop();
                }
            } else if line.chars().next().unwrap().is_numeric() {
                let current_dir_str = current_dir.join("");
                let file_size = line
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                directories
                    .clone()
                    .keys()
                    .filter(|key| current_dir_str.starts_with(*key))
                    .for_each(|key| {
                        if let Some(current_size) = directories.get_mut(key) {
                            *current_size += file_size;
                        }
                    });
            }
        }

        let mut sum: usize = 0;
        let total_size = directories.get("/").unwrap();
        let space_needed = 30_000_000 - (70_000_000 - total_size);

        let mut sizes = directories
            .iter()
            .map(|(_, size)| *size)
            .collect::<Vec<usize>>();
        sizes.sort();

        let mut size_to_be_cleared = 0;

        for size in sizes.iter() {
            if *size <= 100_000 {
                sum += size;
            }

            if *size >= space_needed && size_to_be_cleared == 0 {
                size_to_be_cleared = *size;
            }
        }
        
        assert_eq!(sum, 1084134);
    }

    #[test]
    fn day7_part2() {
        let contents =
            fs::read_to_string("input.txt").expect("Somet1`hing went wrong reading the file");
    }
}