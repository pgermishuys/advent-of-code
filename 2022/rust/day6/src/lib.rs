#[cfg(test)]
mod tests {
    use std::{fs, collections::HashSet, vec::Vec};
    use substring::Substring;

    fn detect_start_packet_marker(message: String, start: usize, distinct_count: usize) -> (usize, String) {
        let mut unique_index = 0;
        let mut buffer: Vec<char> = Vec::new();
        for (index, character) in message.char_indices() {
            if index <= start {
                continue;
            }
            buffer.insert(0, character);
            println!("buffer: {:?}", buffer);
            if buffer.len() >= distinct_count {
                //check uniqueness
                if has_unique_elements(&buffer) {
                    unique_index = index;
                    break;
                }
                buffer.pop();
                continue;
            }
        }
        return (unique_index, message.to_string().substring(unique_index - distinct_count + 1, message.len()).to_owned())
    }

    fn has_unique_elements(items: &Vec<char>) -> bool
    {
        let mut unique: HashSet<char> = HashSet::new();
        items.into_iter().all(move |x| unique.insert(*x))
    }

    #[test]
    fn day6_part1() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");
        let mut index = 0;
        for line in contents.lines() {
            let (start_packet_index, _) = detect_start_packet_marker(line.to_string(), 0, 4);
            index = start_packet_index;
        }

        println!("index: {}", index);

        assert_eq!(index, 1299);
    }

    #[test]
    fn day6_part2() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let distinct_count_for_packet_marker = 4;
        let mut start_packet_marker_index = 0;
        let mut start_message_marker_index = 0;
        for line in contents.lines() {
            let (start_packet_marker, _) = detect_start_packet_marker(line.to_string(), 0, distinct_count_for_packet_marker);
            start_packet_marker_index = start_packet_marker;
            let (start_message_marker, _) = detect_start_packet_marker(line.to_string(), start_packet_marker - start_packet_marker, 14);
            start_message_marker_index = start_message_marker;
        }

        println!("start packet marker: {}", start_packet_marker_index);
        println!("start message marker: {}", start_message_marker_index);

        assert_eq!(start_packet_marker_index, 1299);
        assert_eq!(start_message_marker_index, 3985);
    }
}