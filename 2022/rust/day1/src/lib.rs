#[cfg(test)]
mod tests {
    use std::fs;
    use std::collections::HashMap;
    fn collect_calories_per_elf(contents: String) -> HashMap<i32, i32> {
        let mut calories_per_elf: HashMap<i32, i32> = HashMap::new();

        let mut elf = 0;
        
        for line in contents.lines() {

            if line.is_empty() {
                elf+=1;
                continue;
            }

            let calories = line.parse::<i32>().unwrap();
            calories_per_elf.entry(elf).and_modify(|c| *c +=calories).or_insert(calories);
        }
        return calories_per_elf;
    }

    #[test]
    fn day1_part1() {

        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let calories_per_elf = collect_calories_per_elf(contents);

        let max_key_value = calories_per_elf.iter().max_by_key(|entry | entry.1).unwrap();
        assert_eq!(*max_key_value.1, 75622);
    }

    #[test]
    fn day1_part2() {

        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let calories_per_elf = collect_calories_per_elf(contents);

        let mut calories_per_elf_collection: Vec<(&i32, &i32)> = calories_per_elf.iter().collect();
        calories_per_elf_collection.sort_by(|a, b| b.1.cmp(a.1));
        let top3:i32 = calories_per_elf_collection.iter().take(3).fold(0, | acc, x| acc + x.1);
        assert_eq!(top3, 213159);
    }
}