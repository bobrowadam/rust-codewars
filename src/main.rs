use std::convert::TryInto;

fn main() {
    println!("{}", descending_order(453216));
}

fn descending_order(x: u64) -> u64 {
    let mut splited_number: Vec<u64> = split_int(x);
    splited_number.sort();
    splited_number.reverse();
    join_int_vec(splited_number)
}

fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut mutb = b.to_vec();

    let mut multi_vector: Vec<i64> = a.into_iter().map(|n| (n * n)).collect();
    multi_vector.sort();
    mutb.sort();

    return multi_vector == mutb;
}

fn split_int(int: u64) -> Vec<u64> {
    let mut int_vector: Vec<u64> = Vec::new();
    let mut mutable_int = int;

    while mutable_int > 0 {
        int_vector.push(mutable_int % 10);
        mutable_int = mutable_int / 10;
    }

    int_vector
}

fn join_int_vec(int_vec: Vec<u64>) -> u64 {
    let mut index = int_vec.len();
    let iterable_int_vec: u64 = int_vec
        .into_iter()
        .map(|elm| {
            let result = elm * u64::pow(10, (index - 1).try_into().unwrap());
            index -= 1;
            result
        })
        .sum();
    iterable_int_vec
}

#[test]
fn returns_expected() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}

#[test]
fn tests_comp() {
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![
        11 * 11,
        121 * 121,
        144 * 144,
        19 * 19,
        161 * 161,
        19 * 19,
        144 * 144,
        19 * 19,
    ];
    assert_eq!(comp(a1, a2), true);

    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![
        11 * 21,
        121 * 121,
        144 * 144,
        19 * 19,
        161 * 161,
        19 * 19,
        144 * 144,
        19 * 19,
    ];
    assert_eq!(comp(a1, a2), false);

    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![121, 14641, 20736, 36100, 25921, 361, 20736, 361];
    assert_eq!(comp(a1, a2), false);

    let a1 = vec![2, 2, 3];
    let a2 = vec![4, 9, 9];
    assert_eq!(comp(a1, a2), false);

    let a1 = vec![-121, -144, 19, -161, 19, -144, 19, -11];
    let a2 = vec![121, 14641, 20736, 361, 25921, 361, 20736, 361];
    assert_eq!(comp(a1, a2), true);
}
