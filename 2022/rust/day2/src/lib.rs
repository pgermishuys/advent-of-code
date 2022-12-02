#[cfg(test)]
mod tests {
    use std::fs;
    // A, X for Rock
    // B, Y for Paper
    // C, Z for Scissors

    #[derive(PartialEq, Clone, Copy, Debug)]
    enum Shape {
        Rock,
        Paper,
        Scissors
    }

    fn determine_shape(shape: &str) -> Shape {
        if shape == "A" || shape == "X"{
            return Shape::Rock;
        }
        if shape == "B" || shape == "Y"{
            return Shape::Paper;
        }
        if shape == "C" || shape == "Z"{
            return Shape::Scissors;
        }

        panic!("Could not determine shape");
    }

    fn apply_strategy(strategy: &str, shape: &Shape) -> Shape {
        if strategy == "X" { //loose
            if shape == &Shape::Rock {
                return Shape::Scissors
            }
            if shape == &Shape::Paper {
                return Shape::Rock
            }
            if shape == &Shape::Scissors {
                return Shape::Paper
            }
        }

        if strategy == "Y" { //draw
            return shape.to_owned();
        }

        if strategy == "Z" { //win
            if shape == &Shape::Rock {
                return Shape::Paper
            }
            if shape == &Shape::Paper {
                return Shape::Scissors
            }
            if shape == &Shape::Scissors {
                return Shape::Rock
            }
        }

        panic!("Failed to apply strategy ({:?}) with shape ({:?})", strategy, shape);
    }

    fn play_hand(yours: Shape, mine: Shape) -> i32 {
        let mut score = 0;
        if yours == Shape::Rock {
            //win
            if mine == Shape::Paper {
                println!("win");
                score += 2 + 6;
            //draw
            } else if mine == Shape::Rock {
                println!("draw");
                score += 1 + 3;
            } else if mine == Shape::Scissors {
                println!("loss");
                score += 3 + 0;
            }
        }
        if yours == Shape::Paper {
            //win
            if mine == Shape::Scissors {
                println!("win");
                score += 3 + 6;
            //draw
            } else if mine == Shape::Paper {
                println!("draw");
                score += 2 + 3;
            } else if mine == Shape::Rock {
                println!("loss");
                score += 1 + 0;
            }
        }
        if yours == Shape::Scissors {
            //win
            if mine == Shape::Rock {
                println!("win");
                score += 1 + 6;
            //draw
            } else if mine == Shape::Scissors {
                println!("draw");
                score += 3 + 3;
            } else if mine == Shape::Paper {
                println!("loss");
                score += 2 + 0;
            }
        }
        return score;
    }

    #[test]
    fn day2_part1() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let mut score = 0;
        for line in contents.lines() {
            println!("Hand: {:?}", line);
            let round = line.split_whitespace().take(3).collect::<Vec<&str>>();
            let yours = determine_shape(round[0]);
            let mine = determine_shape(round[1]);

            score += play_hand(yours, mine);
        }

        assert_eq!(score, 13565);
    }

    #[test]
    fn day2_part2() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");

        let mut score = 0;
        for line in contents.lines() {
            println!("Hand: {:?}", line);
            let round = line.split_whitespace().take(3).collect::<Vec<&str>>();
            let yours = determine_shape(round[0]);
            let mine = apply_strategy(round[1], &yours);

            score += play_hand(yours, mine);
        }
        assert_eq!(score, 12424);
    }
}