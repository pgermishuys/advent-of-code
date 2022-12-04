#[cfg(test)]
mod tests {
    use std::{fs, collections::HashSet};

    #[derive(Debug)]
    struct Pair {
        min: i32,
        max: i32
    }

    impl Pair {
        //From str in the format "2-4"
        fn from(str: &str) -> Self {
            let split: Vec<&str> = str.split("-").collect();
            Pair {
                min: split[0].parse::<i32>().unwrap(),
                max: split[1].parse::<i32>().unwrap()
            }
        }

        //Determines whether the one pair completely overlaps the other pair
        fn completely_overlaps(&self, other: &Pair) -> bool {
            self.min <= other.min && self.max >= other.max
        }

        fn overlaps_at_all(&self, other: &Pair) -> bool {
            let mut overlapping_set: HashSet<i32> = HashSet::new();
            for i in self.min..=self.max {
                overlapping_set.insert(i);
            }
            for i in other.min..=other.max {
                if overlapping_set.contains(&i) {
                    return true 
                }
            }
            false
        }
    }

    #[test]
    fn day4_part1() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        //find completely overlapping pairs
        let mut complete_overlapping_pairs = 0;

        for line in contents.lines() {
            let split: Vec<&str> = line.split(",").collect();

            let first_pair = Pair::from(&split[0]);
            let second_pair = Pair::from(&split[1]);

            let completely_overlaps = 
                first_pair.completely_overlaps(&second_pair) || 
                second_pair.completely_overlaps(&first_pair);

            if completely_overlaps {
                complete_overlapping_pairs += 1;
            }
        }

        assert_eq!(complete_overlapping_pairs, 498);
    }

    #[test]
    fn day4_part2() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let mut complete_overlapping_pairs = 0;

        for line in contents.lines() {
            let split: Vec<&str> = line.split(",").collect();

            let first_pair = Pair::from(&split[0]);
            let second_pair = Pair::from(&split[1]);

            let completely_overlaps = 
                first_pair.overlaps_at_all(&second_pair) || 
                second_pair.overlaps_at_all(&first_pair);

            if completely_overlaps {
                complete_overlapping_pairs += 1;
            }
        }
        assert_eq!(complete_overlapping_pairs, 859);
    }
}