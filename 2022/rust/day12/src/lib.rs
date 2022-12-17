#[cfg(test)]
mod tests {
    use std::{env, fmt, fs, process};
    use std::collections::HashMap;
    use pathfinding::prelude::bfs;

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Coordinate {
        pub row: usize,
        pub col: usize,
    }

    impl Coordinate {
        fn from(row: usize, col: usize) -> Coordinate {
            Coordinate { row, col }
        }
    }

    impl fmt::Display for Coordinate {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.row, self.col)
        }
    }

    pub struct Graph {
        pub grid: HashMap<Coordinate, char>,
        pub start_loc: Option<Coordinate>,
        pub end_loc: Option<Coordinate>,
        pub extents: (usize, usize),
    }

    impl Graph {
        pub fn new() -> Graph {
            Graph {
                grid: HashMap::<Coordinate, char>::new(),
                start_loc: None,
                end_loc: None,
                extents: (0, 0),
            }
        }

        pub fn from(s: &String) -> Graph {
            let lines = s.lines();
            let mut grid: HashMap::<Coordinate, char> = HashMap::<Coordinate, char>::new();
            let mut start_loc: Option<Coordinate> = None;
            let mut end_loc: Option<Coordinate> = None;
            let mut row: usize = 0;
            let mut col: usize = 0;
            lines.for_each(|line| {
                let line_chars: Vec<char> = line.chars().collect();
                for c in line_chars.iter() {
                    let coord = Coordinate{ row, col };
                    match c {
                        'S' => {
                            start_loc = Some(coord);
                            grid.insert(coord, 'a');
                        }
                        'E' => {
                            end_loc = Some(coord);
                            grid.insert(coord, 'z');
                        }
                        'a'..='z' => {
                            grid.insert(coord, *c);
                        }
                        _ => { panic!("main(): unexpected char in input!"); }
                    }
                    col += 1;
                }
                row += 1;
                col = 0;
            });
            let max_row: usize = grid.keys().map(|c| c.row).max().unwrap();
            let max_col: usize = grid.keys().map(|c| c.col).max().unwrap();
            let extents: (usize, usize) = (max_row, max_col);
            Graph{ grid, start_loc, end_loc, extents }
        }

        pub fn vertices(&self) -> Vec<Coordinate> {
            let mut vertices: Vec<Coordinate> = self.grid.keys().cloned().collect();
            vertices.sort();
            vertices
        }

        pub fn elevation(c: char) -> i32 {
            let elev: HashMap<char, i32> = 
                ('a'..='z').collect::<Vec<char>>()
                .into_iter()
                .zip((1..=26).collect::<Vec<i32>>()
                .into_iter()).collect();
            elev[&c]
        }

        pub fn neighbors(&self, loc: Coordinate) -> Vec<Coordinate> {
            let mut adjacents: Vec<Coordinate> = vec![];
            let row = loc.row;
            let col = loc.col;
            if ((row as i32) - 1) >= 0 {
                adjacents.push(Coordinate::from(row - 1, col));
            }
            if (row + 1) <= self.extents.0 {
                adjacents.push(Coordinate::from(row + 1, col));
            }
            if ((col as i32) - 1) >= 0 {
                adjacents.push(Coordinate::from(row, col - 1));
            }
            if (col + 1) <= self.extents.1 {
                adjacents.push(Coordinate::from(row, col + 1));
            }
            let valid_neighbors: Vec<Coordinate> = adjacents.into_iter()
                .filter(|x| Graph::elevation(self.grid[x]) <= Graph::elevation(self.grid[&loc]) + 1)
                .collect();
            valid_neighbors
        }
    }

    #[test]
    fn day12_part1() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");
        
        let graph: Graph = Graph::from(&contents);
        let path = bfs(
            &(graph.start_loc.unwrap()),
            |v| graph.neighbors(*v),
            |v| v == &(graph.end_loc.unwrap())
        ).unwrap();

        println!("path length = {}", path.len() - 1);

        assert_eq!(path.len() - 1, 447);
    }

    #[test]
    fn day12_part2() {
        let contents =
            fs::read_to_string("input.txt").expect("Something went wrong reading the file");
        
        println!("{:?}", contents);

        let graph: Graph = Graph::from(&contents);

        let start_nodes: Vec<Coordinate> = graph.vertices()
            .into_iter()
            .filter(|x| Graph::elevation(graph.grid[x]) == 1)
            .collect();

        let mut lengths: Vec<_> = vec![];
        start_nodes.into_iter()
            .for_each(|start_node| {
                let result = bfs(
                    &start_node,
                    |v| graph.neighbors(*v),
                    |v| v == &(graph.end_loc.unwrap())
                );
                match result {
                    None => { /* destination unreachable from this start */ }
                    Some(path) => { lengths.push(path.len() - 1); }
                }
            });
        let min_length = lengths.into_iter().min().unwrap() as u32;

        assert_eq!(min_length, 446);
    }
}