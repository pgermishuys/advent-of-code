#[cfg(test)]
mod tests {
    use std::{fs, collections::HashMap, collections::BTreeMap};

    use substring::Substring;

    fn perform_operation(id: u64, item: u64, operation: &str) -> u64 {
        let parts: Vec<&str> = operation.split_whitespace().collect();
        let operand = parts[1];
        let mut value = item;
        if let Ok(result) = parts[2].parse::<u64>() {
            value = result;
        }
        match operand {
            "*" => {
                return item * value;
            }
            "+" => {
                return item + value;
            }
            "-" => {
                return item - value;
            }
            _ => {
                return 0;
            }//
        }
    }

    fn perform_magic_trick(id: u64, tests: &Vec<&str>) -> u64 {
        let mut items: Vec<u64> = Vec::new();
        for test in tests {
            let parts: Vec<&str> = test.split_whitespace().collect();
            let value = parts[2].parse::<u64>().unwrap();
            items.push(value);
        }
        return items.iter().map(|m| m).product();
    }

    fn perform_test(id: u64, item: &u64, test: &str) -> bool {
        let parts: Vec<&str> = test.split_whitespace().collect();
        let value = parts[2].parse::<u64>().unwrap();
        if *item == 0 {
            return false;
        }
        let result = item % value == 0;
        return result;
    }

    fn which_monkey(id: u64, test_pass: bool, true_action: &str, false_action: &str) -> u64 {
        if test_pass {
            let parts: Vec<&str> = true_action.split_whitespace().collect();
            return parts[3].parse::<u64>().unwrap();
        } else {
            let parts: Vec<&str> = false_action.split_whitespace().collect();
            return parts[3].parse::<u64>().unwrap();
        }
    }

    #[test]
    fn day11_part1() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let mut ids: Vec<u64> = Vec::new();
        let mut items: HashMap<u64, Vec<u64>> = HashMap::new();
        let mut items_checked: HashMap<u64, u64> = HashMap::new();
        let mut operations: HashMap<u64, &str> = HashMap::new();
        let mut tests: HashMap<u64, &str> = HashMap::new();
        let mut true_actions: HashMap<u64, &str> = HashMap::new();
        let mut false_actions: HashMap<u64, &str> = HashMap::new();

        let mut current_monkey_id = 0;
        for line in contents.lines() {
            if line.starts_with("Monkey") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                current_monkey_id = parts[1].substring(0, parts[1].len() - 1).parse::<u64>().unwrap();
                ids.push(current_monkey_id);
                items_checked.insert(current_monkey_id, 0);
            } else if line.contains("Starting items"){
                let parts: Vec<&str> = line.split(":").collect();
                let monkey_items: Vec<u64> = parts[1].split(",").into_iter().map(| x| x.replace(" ", "").parse::<u64>().unwrap()).collect();
                items.insert(current_monkey_id, monkey_items.to_vec());
            } else if line.contains("Operation"){
                let parts: Vec<&str> = line.split("=").collect();
                operations.insert(current_monkey_id, parts[1].substring(1, parts[1].len()));
            } else if line.contains("Test"){
                let parts: Vec<&str> = line.split(":").collect();
                tests.insert(current_monkey_id, parts[1].substring(1, parts[1].len()));
            } else if line.contains("If true"){
                let parts: Vec<&str> = line.split(":").collect();
                true_actions.insert(current_monkey_id, parts[1].substring(1, parts[1].len()));
            } else if line.contains("If false"){
                let parts: Vec<&str> = line.split(":").collect();
                false_actions.insert(current_monkey_id, parts[1].substring(1, parts[1].len()));
            }
        }

        for _ in 0..20 {
            for id in &ids {
                let mut items_to_throw: HashMap<u64, Vec<u64>> = HashMap::new();

                let monkey_items = items.get_mut(&id).unwrap().to_vec();
                items.get_mut(id).unwrap().clear();

                for item in monkey_items {

                    if let Some(item_count) = items_checked.get_mut(&id) {
                        *item_count += 1;
                    }

                    let result = perform_operation(*id, item, operations.get(&id).unwrap());
                    let worry_level = (result as f32 / 3f32).floor() as u64;
                    let pass = perform_test(*id, &worry_level, tests.get(&id).unwrap());
                    let which_monkey = which_monkey(*id, pass, true_actions.get(&id).unwrap(), false_actions.get(&id).unwrap());

                    if !items_to_throw.contains_key(&which_monkey) {
                        items_to_throw.insert(which_monkey, Vec::new());
                    }
                    if let Some(items) = items_to_throw.get_mut(&which_monkey) {
                        items.push(worry_level);
                    }
                }

                //add transformed items to other monkey's list
                for (id, items_to_add) in items_to_throw {
                    items.get_mut(&id).unwrap().extend(items_to_add);
                }
            }
        }

        let sorted_items: BTreeMap<u64, u64> = items_checked.iter().map(|(k,v)| (*v,*k)).collect();
        let first = sorted_items.iter().nth(sorted_items.len() - 1).unwrap();
        let second = sorted_items.iter().nth(sorted_items.len() - 2).unwrap();
        let result = first.0 * second.0;

        assert_eq!(result, 118674);
    }

    #[test]
    fn day11_part2() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let mut ids: Vec<u64> = Vec::new();
        let mut items: HashMap<u64, Vec<u64>> = HashMap::new();
        let mut items_checked: HashMap<u64, u64> = HashMap::new();
        let mut operations: HashMap<u64, &str> = HashMap::new();
        let mut tests: HashMap<u64, &str> = HashMap::new();
        let mut true_actions: HashMap<u64, &str> = HashMap::new();
        let mut false_actions: HashMap<u64, &str> = HashMap::new();

        let mut current_monkey_id = 0;
        for line in contents.lines() {
            if line.starts_with("Monkey") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                current_monkey_id = parts[1].substring(0, parts[1].len() - 1).parse::<u64>().unwrap();
                ids.push(current_monkey_id);
                items_checked.insert(current_monkey_id, 0);
            } else if line.contains("Starting items"){
                let parts: Vec<&str> = line.split(":").collect();
                let monkey_items: Vec<u64> = parts[1].split(",").into_iter().map(| x| x.replace(" ", "").parse::<u64>().unwrap()).collect();
                items.insert(current_monkey_id, monkey_items.to_vec());
            } else if line.contains("Operation"){
                let parts: Vec<&str> = line.split("=").collect();
                operations.insert(current_monkey_id, parts[1].substring(1, parts[1].len()));
            } else if line.contains("Test"){
                let parts: Vec<&str> = line.split(":").collect();
                tests.insert(current_monkey_id, parts[1].substring(1, parts[1].len()));
            } else if line.contains("If true"){
                let parts: Vec<&str> = line.split(":").collect();
                true_actions.insert(current_monkey_id, parts[1].substring(1, parts[1].len()));
            } else if line.contains("If false"){
                let parts: Vec<&str> = line.split(":").collect();
                false_actions.insert(current_monkey_id, parts[1].substring(1, parts[1].len()));
            }
        }

        for round in 0..10000 {
            for id in &ids {
                let mut items_to_throw: HashMap<u64, Vec<u64>> = HashMap::new();

                let monkey_items = items.get_mut(&id).unwrap().to_vec();
                items.get_mut(id).unwrap().clear();

                for item in monkey_items {

                    if let Some(item_count) = items_checked.get_mut(&id) {
                        *item_count += 1;
                    }

                    let absolute = perform_magic_trick(*id, &tests.values().cloned().collect());

                    let mut worry_level = perform_operation(*id, item, operations.get(&id).unwrap());
                    worry_level %= absolute;
                    let pass = perform_test(*id, &worry_level, tests.get(&id).unwrap());
                    let which_monkey = which_monkey(*id, pass, true_actions.get(&id).unwrap(), false_actions.get(&id).unwrap());

                    if !items_to_throw.contains_key(&which_monkey) {
                        items_to_throw.insert(which_monkey, Vec::new());
                    }
                    if let Some(items) = items_to_throw.get_mut(&which_monkey) {
                        items.push(worry_level);
                    }
                }

                //add transformed items to other monkey's list
                for (id, items_to_add) in items_to_throw {
                    items.get_mut(&id).unwrap().extend(items_to_add);
                }
            }
        }

        println!("items: {:?}", items_checked);

        let sorted_items: BTreeMap<u64, u64> = items_checked.iter().map(|(k,v)| (*v,*k)).collect();
        let first = sorted_items.iter().nth(sorted_items.len() - 1).unwrap();
        let second = sorted_items.iter().nth(sorted_items.len() - 2).unwrap();
        println!("{}, {}", first.0, second.0);
        let result = first.0 * second.0;

        assert_eq!(result, 118674);
    }
}