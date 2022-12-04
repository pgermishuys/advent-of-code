#[cfg(test)]
mod tests {
    use std::{fs, collections::{HashSet}};

    #[test]
    fn day3_part1() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let priority = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
        let mut sum = 0;
        for line in contents.lines() {
            let mut compartment1: Vec<char> = Vec::new();
            let mut compartment2: Vec<char> = Vec::new();
            for (index, character) in line.char_indices() {
                if index < line.len() / 2 {
                    compartment1.push(character);
                }
                if index >= line.len() / 2 {
                    compartment2.push(character);
                }
            }
            let comp1: HashSet<char> = HashSet::from_iter(compartment1.iter().cloned());
            let comp2: HashSet<char> = HashSet::from_iter(compartment2.iter().cloned());
            let intersect = comp1.intersection(&comp2);
            sum += priority.iter().position(|&x| x == intersect.to_owned().into_iter().nth(0).unwrap().to_owned()).unwrap() + 1;
        }
        assert_eq!(sum, 7581);
    }

    #[test]
    fn day3_part2() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let priority = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
        let mut sum = 0;
        let mut compartment_number = 0;
        let mut compartments: Vec<Vec<char>> = Vec::new();
        for line in contents.lines() {
            let mut compartment = Vec::new();
            for character in line.chars() {
                compartment.push(character);
            }
            compartments.push(compartment);
            if compartment_number == 2 {
                compartment_number = 0;

                let comp1: HashSet<char> = HashSet::from_iter(compartments[0].iter().cloned());
                let comp2: HashSet<char> = HashSet::from_iter(compartments[1].iter().cloned());
                let comp3: HashSet<char> = HashSet::from_iter(compartments[2].iter().cloned());

                let intersect = &(&comp1 & &comp2) & &comp3;
                sum += priority.iter().position(|&x| x == intersect.to_owned().into_iter().nth(0).unwrap().to_owned()).unwrap() + 1;

                compartments.clear();
                continue;
            }
            compartment_number += 1;
        }
        assert_eq!(sum, 2525);
    }
}