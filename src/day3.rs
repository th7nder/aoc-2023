use std::{io, fs::File, collections::{HashMap, HashSet}};

use crate::files::read_lines;

fn read_schematic() -> Vec<Vec<char>> {
    let lines: io::Lines<io::BufReader<File>> = read_lines("input/3.txt").unwrap();

    let mut schematic = Vec::new();
    for line in lines {
        if let Ok(line) = line {
            schematic.push(line.chars().collect::<Vec<char>>());
        }
    }
    
    schematic
}

fn is_symbol(schematic: &Vec<Vec<char>>, row_idx: usize, col_idx: usize, rr: i32, cc: i32) -> bool {
    let rows = schematic.len();
    let cols = schematic.get(0).unwrap().len();
    
    if row_idx == 0 && rr == -1 {
        return false
    }
    if col_idx == 0 && cc == -1 {
        return false
    }

    if row_idx == rows - 1 && rr == 1 {
        return false
    }
    if col_idx == cols - 1 && cc == 1 {
        return false
    }

    let row_idx = (row_idx as i32 + rr) as usize;
    let col_idx = (col_idx as i32 + cc) as usize;

    let c = *schematic.get(row_idx).unwrap().get(col_idx).unwrap();
    if c == '.' || c.is_numeric() {
        return false
    }

    true
}


fn is_number(schematic: &Vec<Vec<char>>, 
    map: &HashMap<(usize, usize), (i32, usize, usize)>, 
    row_idx: usize, col_idx: usize, rr: i32, cc: i32) -> (bool, (i32, usize, usize)) {
    let rows = schematic.len();
    let cols = schematic.get(0).unwrap().len();
    
    if row_idx == 0 && rr == -1 {
        return (false, (0, 0, 0));
    }
    if col_idx == 0 && cc == -1 {
        return (false, (0, 0, 0));
    }

    if row_idx == rows - 1 && rr == 1 {
        return (false, (0, 0, 0));
    }
    if col_idx == cols - 1 && cc == 1 {
        return (false, (0, 0, 0));
    }

    let row_idx = (row_idx as i32 + rr) as usize;
    let col_idx = (col_idx as i32 + cc) as usize;

    let c = *schematic.get(row_idx).unwrap().get(col_idx).unwrap();
    if c.is_numeric() {
        return (true, *map.get(&(row_idx, col_idx)).unwrap())
    }

    return (false, (0, 0, 0));
}

// pub fn part1() {
//     let schematic = read_schematic();

//     let mut current_num: Vec<char> = Vec::new();
//     let mut is_adjacent = false;

//     let mut ans = 0;
//     for (row_idx, row) in schematic.iter().enumerate() {
//         for (col_idx, c) in row.iter().enumerate() {
//             if *c == '.' || !(*c).is_numeric()  {
//                 if current_num.len() > 0 && is_adjacent { 
//                     println!("{}", current_num.iter().collect::<String>());
//                     let num = current_num.iter().collect::<String>().parse::<i32>().unwrap();
//                     ans += num;
//                 }
//                 current_num.clear();
//                 is_adjacent = false;
//                 continue;
//             }
//             if c.is_numeric() {
//                 current_num.push(*c);

//                 let m = vec![
//                     (1, 0), 
//                     (-1, 0), 
//                     (0, 1),
//                     (0, -1),

//                     (-1, 1),
//                     (1, 1),
//                     (1, -1),
//                     (-1, -1)
//                 ];
//                 // for (rr, cc) in m {
//                 //     let (is, part) = is_number(&schematic, &map, row_idx, col_idx, rr, cc);
//                 //     if is {
//                 //         nums.pus
//                 //     }
//                 // }

//             }
//         }
//     }

//     println!("part1: {ans}")
// }


pub fn part2() {
    let schematic = read_schematic();

    let mut current_num: Vec<char> = Vec::new();

    let mut parts: Vec<i32> = Vec::new();

    let mut map: HashMap<(usize, usize), (i32, usize, usize)> = HashMap::new();

    for (row_idx, row) in schematic.iter().enumerate() {
        for (col_idx, c) in row.iter().enumerate() {
            if *c == '.' || !(*c).is_numeric()  {
                if current_num.len() > 0 { 
                    // println!("herexxxx");
                    println!("{}", current_num.iter().collect::<String>());
                    let num = current_num.iter().collect::<String>().parse::<i32>().unwrap();
                    // THIS IS A PART
                    parts.push(num);

                    if col_idx == 0 {
                        continue;
                    }
                    let mut last_idx = col_idx - 1;
                    let mut col_idx = col_idx - 1;
                    loop {
                        let c = *schematic.get(row_idx).unwrap().get(col_idx).unwrap();

                        if c == '.' || !c.is_numeric() {
                            break
                        }
                        map.insert((row_idx, col_idx), (num, last_idx, row_idx));

                        if col_idx == 0 {
                            break
                        }
                        col_idx -= 1;
                    }


                }
                current_num.clear();
                continue;
            }
            if c.is_numeric() {
                current_num.push(*c)
            }
        }

        if current_num.len() > 0 {
            // println!("here!!!!");
            println!("{}", current_num.iter().collect::<String>());
            let num = current_num.iter().collect::<String>().parse::<i32>().unwrap();
            // THIS IS A PART
            parts.push(num);
            current_num.clear();

            let mut col_idx = schematic.get(0).unwrap().len() - 1;
            let last_idx = col_idx;
            loop {
                let c = *schematic.get(row_idx).unwrap().get(col_idx).unwrap();

                if c == '.' || !c.is_numeric() {
                    break
                }
                map.insert((row_idx, col_idx), (num, last_idx, row_idx));

                if col_idx == 0 {
                    break
                }
                col_idx -= 1;
            }

        }
    }


    
    let mut ans = 0;
    for (row_idx, row) in schematic.iter().enumerate() {
        for (col_idx, c) in row.iter().enumerate() {
            if *c == '*' {
                println!("GEAR! {}, {}", row_idx, col_idx);

                let m = vec![
                    (1, 0), 
                    (-1, 0), 
                    (0, 1),
                    (0, -1),

                    (-1, 1),
                    (1, 1),
                    (1, -1),
                    (-1, -1)
                ];

                let mut gears = HashSet::new();
                for (rr, cc) in m {
                     
                    // println!("checking: {rr} {cc}");
                    let (is, part) = is_number(&schematic, &map, row_idx, col_idx, rr, cc);
                    if is {
                        gears.insert(part);
                    }
                }

                let mut sum = 1;
                if gears.len() == 2 {
                    for (num, idx, x) in gears {
                        sum *= num;
                    }
                    println!("WE GOT IT BOYZ: {:?}", sum);
                    ans += sum;
                }
            }
        }
    }

    // println!("part2: {:?}", map);
    // println!("part2: {}", map.get(&(2 as usize, 2 as usize)).unwrap());
    println!("part2: {ans}");
}