#[cfg(test)]
mod tests {
    use std::{fs, collections::HashSet, iter, env::current_exe};

    #[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
    struct Point {
        x: i32,
        y: i32
    }

    impl Point {
        fn from(x: i32, y: i32) -> Self {
            Point {
                x,
                y
            }
        }

        fn advance(&mut self, direction: &str) {
            match direction {
                "R" => self.x += 1,
                "L" => self.x -= 1,
                "U" => self.y -= 1,
                "D" => self.y += 1,
                _ => panic!("help")
            }
        }

        fn follow(&mut self, head: &Point) {
            let dx = head.x - self.x;
            let dy = head.y - self.y;

            if dy == 2 {
                self.y += 1;
                if dx > 0 {
                    self.x += 1;
                }
                if dx < 0 {
                    self.x += -1;
                }
            } else if dy == -2 {
                self.y -= 1;
                if dx > 0 {
                    self.x += 1;
                }
                if dx < 0 {
                    self.x += -1;
                }
            } else if dx == 2 {
                self.x += 1;
                if dy > 0 {
                    self.y += 1;
                }
                if dy < 0 {
                    self.y += -1;
                }
            } else if dx == -2 {
                self.x -= 1;
                if dy > 0 {
                    self.y += 1;
                }
                if dy < 0 {
                    self.y += -1;
                }
            }
        }
    }

    #[test]
    fn day9_part1() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let mut visited: HashSet<Point>  = HashSet::new();
        let mut head = Point::from(0, 0);
        let mut tail = Point::from(0, 0);
        for line in contents.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let direction = parts[0];
            let distance = parts[1].parse::<i32>().unwrap();

            println!("direction: {}, distance: {}", direction, distance);

            for _ in 0..distance {
                head.advance(direction);

                tail.follow(&head);

                visited.insert(tail);
            }
        }
        assert_eq!(visited.len(), 5907);
    }

    #[test]
    fn day9_part2() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let mut visited: HashSet<Point>  = HashSet::new();
        let mut head = Point::from(0, 0);
        let mut tails: Vec<Point> = vec![
            Point::from(0, 0),
            Point::from(0, 0),
            Point::from(0, 0),
            Point::from(0, 0),
            Point::from(0, 0),
            Point::from(0, 0),
            Point::from(0, 0),
            Point::from(0, 0),
            Point::from(0, 0)
        ];
        for line in contents.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let direction = parts[0];
            let distance = parts[1].parse::<i32>().unwrap();

            for _ in 0..distance {
                head.advance(direction);

                print!("{},{} => ", head.x, head.y);

                let mut current_head = head;
                for index in 0..tails.len() {
                    let tail = &mut tails[index];
                    tail.follow(&current_head);
                    current_head = *tail;
                }
                visited.insert(tails[8]);
            }
        }
        assert_eq!(visited.len(), 36);
    }
}