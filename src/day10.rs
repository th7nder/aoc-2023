use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io, vec,
};

use crate::files::read_lines;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Direction {
    None,
    North,
    NorthWest,
    NorthEast,
    East,
    South,
    SouthEast,
    SouthWest,
    West,
}

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::None => todo!(),
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::NorthWest => todo!(),
            Direction::NorthEast => todo!(),
            Direction::SouthEast => todo!(),
            Direction::SouthWest => todo!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    None,
    NorthAndSouth,
    EastAndWest,
    NorthAndEast,
    NorthAndWest,
    SouthAndWest,
    SouthAndEast,
    Ground,
    StartingPosition,
}

impl Tile {
    fn print(self) -> char {
        match self {
            // ─│┌┐└┘
            Tile::NorthAndSouth => '│',
            Tile::EastAndWest => '─',
            Tile::NorthAndEast => '└',
            Tile::NorthAndWest => '┘',
            Tile::SouthAndWest => '┐',
            Tile::SouthAndEast => '┌',
            Tile::Ground => ' ',
            Tile::StartingPosition => 'S',
            _ => panic!("unkown!"),
        }
    }

    fn parse(c: char) -> Tile {
        match c {
            '|' => Tile::NorthAndSouth,
            '-' => Tile::EastAndWest,
            'L' => Tile::NorthAndEast,
            'J' => Tile::NorthAndWest,
            '7' => Tile::SouthAndWest,
            'F' => Tile::SouthAndEast,
            '.' => Tile::Ground,
            'S' => Tile::StartingPosition,
            _ => panic!("unkown!"),
        }
    }

    fn directions(self) -> Vec<Direction> {
        match self {
            Tile::NorthAndSouth => vec![Direction::North, Direction::South],
            Tile::EastAndWest => vec![Direction::East, Direction::West],
            Tile::NorthAndEast => vec![Direction::North, Direction::East],
            Tile::NorthAndWest => vec![Direction::North, Direction::West],
            Tile::SouthAndWest => vec![Direction::South, Direction::West],
            Tile::SouthAndEast => vec![Direction::South, Direction::East],
            Tile::Ground => vec![],
            Tile::StartingPosition => todo!(),
            Tile::None => vec![],
        }
    }

    fn by_directions(dirs: &Vec<Direction>) -> Tile {
        for t in vec![
            Tile::NorthAndSouth,
            Tile::EastAndWest,
            Tile::NorthAndEast,
            Tile::NorthAndWest,
            Tile::SouthAndWest,
            Tile::SouthAndEast,
        ] {
            let mut c = 0;
            for dir in t.directions() {
                if dirs.contains(&dir) {
                    c += 1
                }
            }
            if c == 2 {
                return t;
            }
        }

        panic!("NONE")
    }
}

fn parse() -> Map {
    let lines: io::Lines<io::BufReader<File>> = read_lines("input/10.txt").unwrap();

    let mut tiles: Vec<Vec<Tile>> = Vec::new();

    for line in lines {
        let mut line_of_tiles = Vec::new();
        if let Ok(line) = line {
            for char in line.chars() {
                line_of_tiles.push(Tile::parse(char));
            }
        }

        tiles.push(line_of_tiles);
    }

    Map::new(tiles)
}

struct Map {
    rows: usize,
    cols: usize,
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
}

impl Map {
    fn new(tiles: Vec<Vec<Tile>>) -> Map {
        let mut map = Map {
            rows: tiles.len(),
            cols: tiles[0].len(),
            tiles,
            start: (0, 0),
        };
        map.determine_start();

        map
    }

    fn determine_start(&mut self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                match self.get(r, c, Direction::None) {
                    Tile::StartingPosition => {
                        let mut required: Vec<Direction> = Vec::new();
                        if self
                            .get(r, c, Direction::North)
                            .directions()
                            .contains(&Direction::South)
                        {
                            required.push(Direction::North);
                        }
                        if self
                            .get(r, c, Direction::South)
                            .directions()
                            .contains(&Direction::North)
                        {
                            required.push(Direction::South);
                        }
                        if self
                            .get(r, c, Direction::West)
                            .directions()
                            .contains(&Direction::East)
                        {
                            required.push(Direction::West);
                        }
                        if self
                            .get(r, c, Direction::East)
                            .directions()
                            .contains(&Direction::West)
                        {
                            required.push(Direction::East);
                        }

                        let new_tile = Tile::by_directions(&required);
                        // println!("{:?}", new_tile);

                        self.start = (r, c);
                        self.tiles[r][c] = new_tile;
                    }
                    _ => {}
                }
            }
        }
    }

    fn get(&self, r: usize, c: usize, dir: Direction) -> Tile {
        if (dir == Direction::North || dir == Direction::NorthEast || dir == Direction::NorthWest)
            && r == 0
        {
            return Tile::None;
        }
        if (dir == Direction::South || dir == Direction::SouthEast || dir == Direction::SouthWest)
            && r == self.rows - 1
        {
            return Tile::None;
        }
        if (dir == Direction::East || dir == Direction::SouthEast || dir == Direction::NorthEast)
            && c == self.cols - 1
        {
            return Tile::None;
        }
        if (dir == Direction::West || dir == Direction::SouthWest || dir == Direction::NorthWest)
            && c == 0
        {
            return Tile::None;
        }

        let (dr, dc) = match dir {
            Direction::None => (0, 0),
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            Direction::NorthWest => (-1, -1),
            Direction::NorthEast => (-1, 1),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (1, -1),
        };

        // println!("BEF r: {} c: {} rows: {} cols: {} {:?}, dr: {}, dc: {}", r, c, self.rows, self.cols, dir, dr, dc);

        let r = (r as i32 + dr) as usize;
        let c = (c as i32 + dc) as usize;

        // println!("r: {} c: {} rows: {} cols: {} {:?}, dr: {}, dc: {}", r, c, self.rows, self.cols, dir, dr, dc);
        *self.tiles.get(r).unwrap().get(c).unwrap()
    }

    fn get_xy(&self, r: usize, c: usize, dir: Direction) -> (Tile, usize, usize) {
        if (dir == Direction::North || dir == Direction::NorthEast || dir == Direction::NorthWest)
            && r == 0
        {
            return (Tile::None, 0, 0);
        }
        if (dir == Direction::South || dir == Direction::SouthEast || dir == Direction::SouthWest)
            && r == self.rows - 1
        {
            return (Tile::None, 0, 0);
        }
        if (dir == Direction::East || dir == Direction::SouthEast || dir == Direction::NorthEast)
            && c == self.cols - 1
        {
            return (Tile::None, 0, 0);
        }
        if (dir == Direction::West || dir == Direction::SouthWest || dir == Direction::NorthWest)
            && c == 0
        {
            return (Tile::None, 0, 0);
        }
        let (dr, dc) = match dir {
            Direction::None => (0, 0),
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            Direction::NorthWest => (-1, -1),
            Direction::NorthEast => (-1, 1),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (1, -1),
        };

        let r = (r as i32 + dr) as usize;
        let c = (c as i32 + dc) as usize;

        (*self.tiles.get(r).unwrap().get(c).unwrap(), r, c)
    }

    fn traverse(&mut self, start: (usize, usize)) {
        let mut queue = VecDeque::new();
        queue.push_back(start);
        let mut distance = 0;

        let mut distances = HashMap::new();

        let mut visited = HashSet::new();
        visited.insert(start);

        while queue.len() > 0 {
            let tiles = queue.len();
            for _ in 0..tiles {
                let (r, c) = queue.pop_front().unwrap();
                let s = self.get(r, c, Direction::None);
                distances.insert((r, c), distance);

                // self.print(&visited);
                // println!("-------------");
                // println!("Trying out: {} {}, {:?}", r + 1, c + 2, s.directions());
                for dir in s.directions() {
                    let (next, nr, nc) = self.get_xy(r, c, dir);
                    // println!("Next? {} {}, {:?}", nr + 1, nc + 2, dir);
                    // next.directions().contains(&dir.opposite())
                    if !visited.contains(&(nr, nc)) {
                        // println!("Can go to: {}, {}", nr + 1, nc + 2);
                        visited.insert((nr, nc));
                        queue.push_back((nr, nc));
                    }
                }
            }
            distance += 1;
            // println!("distance {}", distance)
        }

        println!("Part1: {}", distance - 1);


        for r in 0..self.rows {
            for c in 0..self.cols {
                if !visited.contains(&(r, c)) {
                    self.tiles[r][c] = Tile::Ground;
                }
            }
        }

        let mut ans = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.get(r, c, Direction::None) == Tile::Ground {
                    if self.cast((r, c), &visited) % 2 == 1 {
                        ans += 1;
                    }
                }
            }
        }




        println!("Part 2: {ans}");
    }

    fn cast(&self, (r, c): (usize, usize), visited: &HashSet<(usize, usize)>) -> usize {
        let mut count = 0;
        let (mut tile, mut r, mut c) = self.get_xy(r, c, Direction::West);

        while tile != Tile::None {
            if visited.contains(&(r, c)) {
                match tile {
                    Tile::NorthAndSouth => count += 1,
                    // L----J
                    // F----7
                    Tile::NorthAndWest => count += 1, // J
                    Tile::NorthAndEast => count += 1, // L
                    _ => {},
                }    
            }

            (tile, r, c) = self.get_xy(r, c, Direction::West);
        }

        count
    }

    fn print(&self, visited: &HashSet<(usize, usize)>) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                let tile = self.get(r, c, Direction::None);
                if visited.contains(&(r, c)) {
                    // print!("{}\t", distances.get(&(r, c)).unwrap())
                    // print!("{}", tile.print())
                    if tile == Tile::Ground {
                        print!("O")
                    } else {
                        print!("{}", tile.print())
                    }
                    // print!("O")
                } else {
                    if tile == Tile::Ground {
                        print!("I")
                    } else {
                        print!("{}", tile.print());
                    }
                }
            }
            println!();
        }
    }
}

pub fn part1() {
    let mut x = parse();
    x.traverse(x.start);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let map = parse();
        let mut x = parse();
        x.traverse(x.start);
    }
}
