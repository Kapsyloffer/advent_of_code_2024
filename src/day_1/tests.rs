use crate::day_1::advent_1::*;
#[test]
fn test_distance() {
    let mut left: Vec<i64> = [3, 4, 2, 1, 3, 3].to_vec();
    let mut right: Vec<i64> = [4, 3, 5, 3, 9, 3].to_vec();

    let sorted_left = sort_list(&mut left);
    let sorted_right = sort_list(&mut right);

    let dist = calc_dist(&sorted_left, &sorted_right);

    assert_eq!(dist, 11);
}

#[test]
fn test_similarity() {
    let left: Vec<i64> = [3, 4, 2, 1, 3, 3].to_vec();
    let right: Vec<i64> = [4, 3, 5, 3, 9, 3].to_vec();

    let sim = calc_similarity(&left, &right);

    assert_eq!(sim, 31);
}

