#[cfg(test)]
mod tests {
    use std::{fs};

    fn parse_instruction(line: &str) -> (String, i32){
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 1 {
            return (parts[0].to_string(), 0);
        }
        return (parts[0].to_string(), parts[1].parse::<i32>().unwrap());
    }

    fn calculate_signal_strength(x: i32, cycle: usize) -> Option<i32> {
        if cycle == 20 || cycle % 40 == 20 {
            return Some(cycle as i32 * x);
        } 
        return None;
    }

    fn render(x: i32, cycle: usize) {
        print!("{}", "#");
        if cycle % 40 == 0 {
            println!();
        }
    }

    #[test]
    fn day10_part1() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let mut register_x = 1;
        let mut cycle = 0 as usize;
        let mut signal_strengths: Vec<i32> = Vec::new();

        for line in contents.lines() {
            let (instruction, value) = parse_instruction(line);
            match instruction.as_str() {
                "addx" => {
                    cycle += 1;
                    if let Some(strength) = calculate_signal_strength(register_x, cycle) {
                        signal_strengths.push(strength);
                    }
                    cycle += 1;
                    if let Some(strength) = calculate_signal_strength(register_x, cycle) {
                        signal_strengths.push(strength);
                    }
                    register_x += value;
                },
                "noop" => {
                    cycle += 1;
                    if let Some(strength) = calculate_signal_strength(register_x, cycle) {
                        signal_strengths.push(strength);
                    }
                },
                _ => {
                }
            }
        }

        let sum: i32 = signal_strengths.iter().sum();
        assert_eq!(sum, 0);
    }

    fn set_sprite_positions(register_x: i32, sprite_positions: &mut [i32]){
        //reset
        for elem in sprite_positions.iter_mut() { *elem = 0; }
        //draw
        let middle = register_x;
        let left = register_x - 1;
        let right = register_x + 1;
        if middle >= 0 && middle < 40 {
            sprite_positions[middle as usize] = 1;
        }
        if left >= 0 && left < 40 {
            sprite_positions[left as usize] = 1;
        }
        if right >= 0 && right < 40 {
            sprite_positions[right as usize] = 1;
        }
        print!("Sprite position: ");
        for i in sprite_positions {
            if *i == 1 {
                print!("{}", "#");
            } else {
                print!("{}", ".");
            }
        }
        println!();
    }

    fn set_crt_state(cycle: usize, x: i32, row: usize, sprite_positions: [i32; 40], crt_state: &mut [[&str; 40]; 6]) {
        let pixel_position = (cycle - 1) % 40;
        if sprite_positions[pixel_position] == 1 {
            crt_state[row][pixel_position] = "#";
        }
        println!("During cycle  {}: CRT draws pixel in position {}", cycle, pixel_position);
        print!("Current CRT row: ");
        for (_, col) in crt_state[row].iter().enumerate() {
            print!("{}", col);
        }
        println!();
        // println!("pixel position: {}, sprite_positions: {:?}", pixel_position, sprite_positions);
    }

    #[test]
    fn day10_part2() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let mut register_x = 1;
        let mut row = 0;
        let mut cycle = 0 as usize;
        let mut sprite_positions = [0i32; 40];
        let mut crt_state = [["."; 40]; 6];
        set_sprite_positions(register_x, &mut sprite_positions);

        for line in contents.lines() {
            let (instruction, value) = parse_instruction(line);
            match instruction.as_str() {
                "addx" => {
                    cycle += 1;
                    if cycle % 40 == 0 {
                        row += 1;
                    }

                    set_crt_state(cycle, register_x, row, sprite_positions, &mut crt_state);

                    cycle += 1;
                    if cycle % 40 == 0 {
                        row += 1;
                    }

                    set_crt_state(cycle, register_x, row, sprite_positions, &mut crt_state);

                    register_x += value;

                    set_sprite_positions(register_x, &mut sprite_positions);
                },
                "noop" => {
                    cycle += 1;
                    set_crt_state(cycle, register_x, row, sprite_positions, &mut crt_state);
                    
                    if cycle % 40 == 0 {
                        row += 1;
                    }
                },
                _ => {
                }
            }
        }
        
        for (i, row) in crt_state.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                print!("{}", col);
            }
            println!();
        }

    }
}