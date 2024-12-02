static INPUT: &str = include_str!("input.txt");

fn parse() -> Vec<Vec<i32>> {
    INPUT
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn part1() {
    let levels = parse();

    let mut total_safe = 0;

    for level in levels {
        let a = level[0];
        let b = level[1];
        let mode = a.cmp(&b);
        let mut safe = true;

        for slice in level.windows(2) {
            let &[a, b] = slice else { unreachable!() };

            if a.cmp(&b) != mode {
                safe = false;
                break;
            }

            if (a - b).abs() > 3 {
                safe = false;
                break;
            }
        }

        if safe {
            total_safe += 1;
        }
    }

    println!("Part one: {total_safe}");
}

fn part2() {
    let levels = parse();

    let mut total_safe = 0;

    for level in levels {
        for remove in -1..(level.len() as i32) {
            let mut level = level.clone();
            if remove >= 0 {
                level.remove(remove as usize);
            }

            let a = level[0];
            let b = level[1];
            let mode = a.cmp(&b);
            let mut safe = true;

            for slice in level.windows(2) {
                let &[a, b] = slice else { unreachable!() };

                let good = a.cmp(&b) == mode && (a - b).abs() <= 3;
                if !good {
                    safe = false;
                    break;
                }
            }

            if safe {
                total_safe += 1;
                break;
            }
        }
    }

    println!("Part two: {total_safe}");
}

fn main() {
    part1();
    part2();
}
