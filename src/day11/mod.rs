use std::str::FromStr;
use std::collections::HashMap;

pub fn step1(input : String) {
    let serial = i32::from_str(&input).unwrap();
    let grid = build_power_grid(serial);

    let mut max = -5;
    let mut pos = (0, 0);
    for x in 1..298 {
        for y in 1..298 {
            let sum = get_power_sum(&grid, x, y, 3);
            if sum > max {
                max = sum;
                pos = (x, y);
            }
        }
    }
    println!("{},{} = {}", pos.0, pos.1, max);
}

pub fn step2(input : String) {
    let serial = i32::from_str(&input).unwrap();
    let grid = build_power_grid(serial);

    let mut max = -5;
    let mut pos = (0, 0, 0);
    for x in 1..300 {
        for y in 1..300 {
            let mut sum = 0;
            for size in 1..(301 - x).min(301 - y) {
                sum += get_power_edge_sum(&grid, x, y, size);
                if sum > max {
                    max = sum;
                    pos = (x, y, size);
                }
                if size > 10 && sum < 0 {
                    break;
                }
            }
        }
    }
    println!("{},{},{}", pos.0, pos.1, pos.2);
}


fn build_power_grid(serial : i32) -> HashMap<(i32, i32), i32> {
    let mut grid = HashMap::new();
    for x in 1..300 {
        for y in 1..300 {
            grid.insert((x, y), power(x, y, serial));
        }
    }
    grid
}

fn get_power_sum(grid : &HashMap<(i32, i32), i32>, x : i32, y : i32, size: i32) -> i32 {
    let mut sum = 0;
    for ix in 0..size {
        for iy in 0..size {
            sum += grid.get(&(x + ix, y + iy)).unwrap();
        }
    }
    sum
}

fn get_power_edge_sum(grid : &HashMap<(i32, i32), i32>, x : i32, y : i32, size: i32) -> i32 {
    let mut sum = 0;
    for iy in 0..size {
        sum += grid.get(&(x + size - 1, y + iy)).unwrap();
    }
    for ix in 0..(size-1) {
        sum += grid.get(&(x + ix, y + size - 1)).unwrap();
    }
    sum
}

fn power(x : i32, y : i32, serial : i32) -> i32 {
    let rack_id = x + 10;
    let level = (rack_id * y + serial) * rack_id;
    let d = (level % 1000) / 100;
    d - 5
}