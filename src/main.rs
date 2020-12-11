mod test_main;

use std::fs::File;
use std::io::{BufReader, BufRead};
use log::{info, debug};
use std::iter::FromIterator;

fn get_input_data(file_name: &str) -> Vec<Vec<char>> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);

    let mut lines:  Vec<Vec<char>> = Vec::new();

    for line in f.lines() {
        lines.push(line.unwrap().trim().chars().collect())
    }

    return lines;
}

fn pretty_print(area: &Vec<Vec<char>>) -> String {
    let mut out = String::new();
    out += "\n";
    for line in &area[0..area.len()-1] {
        out.push_str(&String::from_iter(line));
        out.push_str("\n")
    }
    out.push_str(&String::from_iter(&area[area.len()-1]));
    return out;
}

fn create_padded_area(area: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = area[0].len() + 2;
    let mut padded_area: Vec<Vec<char>> = vec![("+".repeat(width)).chars().collect()];
    for line in area {
        let mut line = line.clone();
        line.push('+');
        line.insert(0,'+');
        padded_area.push(line);
    }
    padded_area.push(("+".repeat(width)).chars().collect());
    return padded_area;
}

fn count_occupied(padded_area: &Vec<Vec<char>>, y: usize, x: usize) -> i32 {
    let mut count: i32 = 0;
    for n in y-1..y+2 {
        for m in x-1..x+2 {
            if padded_area[n][m] == '#' {
                count += 1
            }
        }
    }
    return count;
}

fn follow_ray(padded_area: &Vec<Vec<char>>, y: usize, x: usize, direction: (i32, i32)) -> bool {
    let mut current_x: usize = ((x as i32) + direction.0) as usize;
    let mut current_y: usize = ((y as i32) + direction.1) as usize;
    while padded_area[current_y][current_x] != '+' {
        if padded_area[current_y][current_x] == '#' {
            return true;
        }
        if padded_area[current_y][current_x] == 'L' {
            return false;
        }
        current_x = ((current_x as i32) + direction.0) as usize;
        current_y = ((current_y as i32) + direction.1) as usize;
    }
    return false;
}

fn count_occupied_part_2(padded_area: &Vec<Vec<char>>, y: usize, x: usize) -> i32 {
    let directions:Vec<(i32, i32)> = vec![(1,0), (0,1), (-1,0), (0,-1),
                                              (1,1), (1,-1), (-1,1), (-1,-1)];
    let mut count = 0;
    for direction in directions {
        debug!("{}, {:?}, {}, {}", count, direction, y, x);
        count += follow_ray(padded_area, y, x, direction) as i32
    }
    debug!("{}", count);
    return count;
}

fn one_round_part_2(area: &mut Vec<Vec<char>>) -> bool {
    let padded_area = create_padded_area(area);
    let height: usize = area.len();
    let width: usize = area[0].len();
    let mut changed: bool = false;
    for n in 1..height+1 {
        for m in 1..width+1 {
            if padded_area[n][m] == '.' {
                continue
            }
            if padded_area[n][m] == 'L' {
                if count_occupied_part_2(&padded_area, n, m) == 0 {
                    changed = true;
                    area[n-1][m-1] = '#';
                }
            }
            if padded_area[n][m] == '#' {
                if count_occupied_part_2(&padded_area, n, m) >= 5 {
                    changed = true;
                    area[n-1][m-1] = 'L';
                }
            }
        }
    }
    debug!("{}", changed);
    return changed;
}

fn one_round(area: &mut Vec<Vec<char>>) -> bool {
    let padded_area = create_padded_area(area);
    let height: usize = area.len();
    let width: usize = area[0].len();
    let mut changed: bool = false;
    for n in 1..height+1 {
        for m in 1..width+1 {
            if padded_area[n][m] == '.' {
                continue
            }
            if padded_area[n][m] == 'L' {
                if count_occupied(&padded_area, n, m) == 0 {
                    changed = true;
                    area[n-1][m-1] = '#';
                }
            }
            if padded_area[n][m] == '#' {
                if count_occupied(&padded_area, n, m) >= 5 {
                    changed = true;
                    area[n-1][m-1] = 'L';
                }
            }
        }
    }
    return changed;
}

fn count_occupied_seats(area: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for line in area {
        for seat in line {
            if seat == &'#' {
                count += 1;
            }
        }
    }
    return count;
}

fn solution_part_1(file_name: &str) -> i32 {
    let mut area = get_input_data(file_name);
    debug!("{}", pretty_print(&area));
    while one_round(&mut area) {
        debug!("{}", pretty_print(&area));
    }
    return count_occupied_seats(&area);
}

fn solution_part_2(file_name: &str) -> i32 {
    let mut area = get_input_data(file_name);
    debug!("{}", pretty_print(&area));
    while one_round_part_2(&mut area) {
        debug!("{}", pretty_print(&area));
    }
    return count_occupied_seats(&area);
}

fn main() {
    env_logger::init();
    info!("{}", solution_part_1("inputData.txt"));
    info!("{}", solution_part_2("inputData.txt"));
}
