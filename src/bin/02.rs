use std::collections::HashMap;

#[aoc2023::main(02)]
fn main(input: &str) -> (usize, usize) {
    let mut p1: usize = 0;
    let mut p2: usize = 0;

    for game in input.lines() {
        let game_info: Vec<&str> = game.split(": ").collect();
        let mut possible = true;
        let mut min_cubes: HashMap<&str, usize> = HashMap::new();

        for subset in game_info[1].split(';') {
            for cube in subset.split(',') {
                let cube_color: Vec<&str> = cube.split_whitespace().collect();
                let color = cube_color[1];
                let count = cube_color[0].parse::<usize>().unwrap();
                let current_min = min_cubes.entry(color).or_insert(count);
                *current_min = (*current_min).max(count);
                match color {
                    "red" if count > 12 => {
                        possible = false;
                    }
                    "green" if count > 13 => {
                        possible = false;
                    }
                    "blue" if count > 14 => {
                        possible = false;
                    }
                    _ => (),
                }
            }
        }
        if possible {
            p1 += game_info[0]
                .split(" ")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();
        }
        let power = min_cubes.values().product::<usize>();
        p2 += power;
    }
    (p1, p2)
}
