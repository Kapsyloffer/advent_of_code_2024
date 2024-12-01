pub fn advent_1() {
    let (mut left, mut right) = load_lists();

    let sorted_left = sort_list(&mut left);
    let sorted_right = sort_list(&mut right);

    let dist = calc_dist(&sorted_left, &sorted_right);
    let sim = calc_similarity(&sorted_left, &sorted_right);

    println!("Part 1: {}", dist);
    println!("Part 2: {}", sim);
}

pub fn calc_dist(list1: &Vec<i64>, list2: &Vec<i64>) -> i64 {
    let mut total_dist = 0;
    for i in 0..list1.len() {
        total_dist += (list1[i] - list2[i]).abs();
    }
    total_dist
}

pub fn calc_similarity(left: &Vec<i64>, right: &Vec<i64>) -> i64 {
    print!("{:?}{:?}\n", left, right);
    let mut sim: i64 = 0;
    for x in 0..left.len() {
        let mut sim_x: i64 = 0;
        for y in 0..right.len() {
            if right[y] == left[x] {
                sim_x += 1;
            }
        }
        sim += sim_x * left[x];
        println!("{}, dök up {} gånger, therefore {} x {} = {}", left[x], sim_x, left[x], sim_x, sim_x * left[x]);
    }
    return sim;
}

pub fn sort_list(list: &mut Vec<i64>) -> Vec<i64> {
    let mut sorted_list = list.clone();
    sorted_list.sort();
    sorted_list
}

use std::fs::read_to_string;

fn load_lists() -> (Vec<i64>, Vec<i64>) {
    let file_path: &str = "points.txt";
    println!("In file {}", file_path);

    let contents = read_to_string(file_path).expect("Should have been able to read the file");

    let mut left = Vec::new();
    let mut right = Vec::new();

    for (line_num, line) in contents.lines().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        match (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
            (Ok(left_num), Ok(right_num)) => {
                left.push(left_num);
                right.push(right_num);
            }
            _ => eprintln!("Line {}: parsing error: {}", line_num + 1, line),
        }
    }

    (left, right)
}

