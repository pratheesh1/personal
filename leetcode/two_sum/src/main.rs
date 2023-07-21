use std::collections::HashMap;

fn two_sum(nums: &Vec<i64>, target: i64) -> Vec<usize> {
    let mut complements: HashMap<i64, usize> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        match complements.get(num) {
            Some(&index) => return vec![index, i],
            None => complements.insert(target - num, i),
        };
    }

    println!("{:?}", complements);
    panic!("No results found!");
}

fn main() {
    let test_1 = two_sum(&vec![2, 7, 11, 15], 9);
    println!("{:?}", test_1);

    let test_1 = two_sum(&vec![3, 2, 4], 6);
    println!("{:?}", test_1);

    let test_1 = two_sum(&vec![3, 3], 6);
    println!("{:?}", test_1);
}
