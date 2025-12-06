const CONTAINERS: [u32; 20] = [
            50, 44, 11, 49, 42, 46, 18, 32, 26, 40, 21, 7, 18, 43, 10, 47, 36, 24, 22, 40,
        ];

pub fn solve1() {
    let all_combinations = count_combinations(
        150,
        &CONTAINERS,
    );
    println!("{}", all_combinations);
}

pub fn solve2() {
    let mut all_min_combinations = 0;
    for i in 1..=CONTAINERS.len() {
        all_min_combinations = count_combinations_limited(150, i, &CONTAINERS);
        if all_min_combinations > 0 {
            break;
        }
    }

    println!("{}", all_min_combinations);
}

fn count_combinations(target: u32, containers: &[u32]) -> u32 {
    if containers.len() == 0 {
        0
    } else if target == containers[0] {
        // Use the first container and be done, or try the others.
        1 + count_combinations(target, &containers[1..])
    } else if target > containers[0] {
        // Either fill the first container, or not.
        count_combinations(target - containers[0], &containers[1..])
            + count_combinations(target, &containers[1..])
    } else
    /* target < containers[0] */
    {
        // Can't fill the first containers, try the other ones.
        count_combinations(target, &containers[1..])
    }
}

fn count_combinations_limited(target: u32, containers_left: usize, containers: &[u32]) -> u32 {
    if containers.len() == 0 {
        0
    } else if containers_left == 0 {
        0
    } else if target == containers[0] {
        // Use the first container and be done, or try the others.
        1 + count_combinations_limited(target, containers_left, &containers[1..])
    } else if target > containers[0] {
        // Either fill the first container, or not.
        count_combinations_limited(target - containers[0], containers_left - 1, &containers[1..])
            + count_combinations_limited(target, containers_left, &containers[1..])
    } else
    /* target < containers[0] */
    {
        // Can't fill the first containers, try the other ones.
        count_combinations_limited(target, containers_left, &containers[1..])
    }
}

#[test]
fn test_count_combinations() {
    assert_eq!(count_combinations(12, &[]), 0);
    assert_eq!(count_combinations(12, &[12]), 1);
    assert_eq!(count_combinations(12, &[5, 7]), 1);
    assert_eq!(count_combinations(12, &[5, 4]), 0);
    assert_eq!(count_combinations(12, &[12, 12]), 2);
    assert_eq!(count_combinations(25, &[20, 15, 10, 5, 5]), 4);

    assert_eq!(
        count_combinations(
            150,
            &CONTAINERS
        ),
        654
    );
}

#[test]
fn test_count_combinations_limited() {
    assert_eq!(count_combinations_limited(12, 1, &[]), 0);
    assert_eq!(count_combinations_limited(12, 0, &[12]), 0);
    assert_eq!(count_combinations_limited(12, 1, &[12]), 1);
    assert_eq!(count_combinations_limited(12, 0, &[5, 7]), 0);
    assert_eq!(count_combinations_limited(12, 1, &[5, 7]), 0);
    assert_eq!(count_combinations_limited(12, 2, &[5, 7]), 1);
    assert_eq!(count_combinations_limited(12, 7, &[5, 4]), 0);
    assert_eq!(count_combinations_limited(12, 1, &[12, 12]), 2);
    assert_eq!(count_combinations_limited(25, 2, &[20, 15, 10, 5, 5]), 3);
}