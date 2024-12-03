use regex::Regex;

static INPUT: &str = include_str!("input.txt");

fn part1() {
    let pattern = Regex::new(r"^mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0i64;
    let mut cursor = 0;

    while cursor < INPUT.len() {
        let slice = &INPUT[cursor..];
        let Some(found) = pattern.captures(slice) else {
            cursor += 1;
            continue;
        };

        let whole = found.get(0).unwrap();
        cursor += whole.len();

        let a: i64 = found[1].parse().unwrap();
        let b: i64 = found[2].parse().unwrap();
        sum += a * b;
    }

    println!("Part one: {sum}");
}

fn part2() {
    let enable = Regex::new(r"^do\(\)").unwrap();
    let disable = Regex::new(r"^don't\(\)").unwrap();
    let mul = Regex::new(r"^mul\((\d+),(\d+)\)").unwrap();

    let mut enabled = true;
    let mut sum = 0i64;
    let mut cursor = 0;

    while cursor < INPUT.len() {
        let slice = &INPUT[cursor..];

        if let Some(found) = enable.find(slice) {
            enabled = true;
            cursor += found.len();
            continue;
        }

        if let Some(found) = disable.find(slice) {
            enabled = false;
            cursor += found.len();
            continue;
        }

        if let Some(found) = mul.captures(slice) {
            let whole = found.get(0).unwrap();
            cursor += whole.len();

            let a: i64 = found[1].parse().unwrap();
            let b: i64 = found[2].parse().unwrap();

            if enabled {
                sum += a * b;
            }
            continue;
        }

        cursor += 1;
    }

    println!("Part two: {sum}");
}

fn main() {
    part1();
    part2();
}
