use std::collections::HashMap;

static INPUT: &str = include_str!("input.txt");

fn lists() -> (Vec<i32>, Vec<i32>) {
    let entries = INPUT.lines().count();

    let mut list1 = Vec::with_capacity(entries);
    let mut list2 = Vec::with_capacity(entries);

    for line in INPUT.lines() {
        let mut nums = line.split_whitespace();
        let a: i32 = nums.next().unwrap().parse().unwrap();
        let b: i32 = nums.next().unwrap().parse().unwrap();
        list1.push(a);
        list2.push(b);
    }

    (list1, list2)
}

fn part1() {
    let (mut list1, mut list2) = lists();
    list1.sort();
    list2.sort();

    let mut sum = 0;

    for (&a, &b) in list1.iter().zip(list2.iter()) {
        sum += (a - b).abs();
    }

    println!("Part one: {sum}");
}

fn part2() {
    let (list1, list2) = lists();

    let mut list2_counts: HashMap<i32, i32> = HashMap::with_capacity(list2.len());
    for &n in &list2 {
        *list2_counts.entry(n).or_default() += 1;
    }

    let mut similarity = 0;

    for &n in &list1 {
        similarity += n * list2_counts.get(&n).copied().unwrap_or_default();
    }

    println!("Part two: {similarity}");
}

fn main() {
    part1();
    part2();
}
