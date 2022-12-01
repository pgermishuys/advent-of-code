#[cfg(test)]
mod tests {
    use std::fs;
    fn determine_floor(item: char) -> i32 {
        if item == '(' {
            return 1;
        }
        if item == ')' {
            return -1;
        }
        return 0;
    }

    #[test]
    fn day1_part1() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");
        assert_eq!(contents, "");
    }
}