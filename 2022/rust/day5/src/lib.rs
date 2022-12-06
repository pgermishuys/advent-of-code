#[cfg(test)]
mod tests {
    use std::{fs, collections::HashSet, vec::Vec, string};

    fn sort_items_into_stacks(contents: &String) -> Vec<Stack> {
        let mut column = 0;
        let mut track_columns = true;
        let mut stacks: Vec<Stack> = Vec::new();
        for line in contents.lines() {
            if !line.contains("[") {
                track_columns = false;
            }
            column = 0;
            if track_columns {
                for (index, character) in line.char_indices() {
                    if index % 4 == 1 {
                        if character != ' ' {
                            if let Some(stack_index) = get_stack_index(column, &stacks) {
                                stacks[stack_index].items.push(character);
                            } else {
                                stacks.push(Stack::from(column, character));
                            }
                        }
                        column += 1;
                    }
                }
            }
        }
        return stacks;
    }

    fn get_stack_index(column: usize, stacks: &Vec<Stack>) -> Option<usize> {
        stacks.iter().position(|x| x.column == column)
    }

    fn perform_instructions(contents: &String, retain_order: bool, stacks: &mut Vec<Stack>) {
        let mut start_tracking_instructions = false;
        for line in contents.lines() {
            if start_tracking_instructions {
                let instruction = Instruction::from(line);
                let mut items_to_move: Vec<char> = Vec::new();
                for _ in 0..instruction.quantity {
                    if let Some(stack_index) = get_stack_index(instruction.from, &stacks) {
                        let item = stacks[stack_index].get_and_remove_first_item();
                        items_to_move.push(item);
                    }
                }
                if !retain_order {
                    items_to_move.reverse();
                }
                for item in items_to_move {
                    if let Some(stack_index) = get_stack_index(instruction.to, &stacks) {
                        stacks[stack_index].items.insert(0, item);
                    }
                }
            }

            if line.is_empty() {
                start_tracking_instructions = true;
            }
        }
    }

    #[derive(Debug)]
    struct Instruction {
        quantity: usize,
        from: usize,
        to: usize
    }

    impl Instruction {
        fn from(line: &str) -> Instruction {
            let split: Vec<&str> = line.split_whitespace().collect();
            Instruction {
                quantity: split[1].parse::<usize>().unwrap(),
                from: split[3].parse::<usize>().unwrap() - 1,
                to: split[5].parse::<usize>().unwrap() - 1
            }
        }
    }

    #[derive(Debug)]
    struct Stack {
        column: usize,
        items: Vec<char>,
    }

    impl Stack {
        fn from(column: usize, item: char) -> Stack {
            Stack {
                column,
                items: vec![item],
            }
        }

        fn get_and_remove_first_item(&mut self) -> char{
            let last_item = self.items.first().unwrap().to_owned();
            self.items.remove(self.items.iter().position(|x| x == &last_item).unwrap());
            return last_item;
        }
    }

    #[test]
    fn day5_part1() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let mut stacks = sort_items_into_stacks(&contents);
        perform_instructions(&contents, true, &mut stacks);
        let mut top_items :Vec<char> = Vec::new();
        for i in 0..stacks.len() {
            let index = get_stack_index(i, &stacks).unwrap();
            let mut items = stacks[index].items.to_owned();
            items.reverse();
            top_items.push(items.last().unwrap().to_owned());
        }

        println!("top items: {:?}", top_items);
    }

    #[test]
    fn day5_part2() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let mut stacks = sort_items_into_stacks(&contents);
        perform_instructions(&contents, false, &mut stacks);
        let mut top_items :Vec<char> = Vec::new();
        for i in 0..stacks.len() {
            let index = get_stack_index(i, &stacks).unwrap();
            let items = stacks[index].items.to_owned();
            top_items.push(items.first().unwrap().to_owned());
        }

        println!("top items: {:?}", top_items);
    }
}