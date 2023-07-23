#![allow(dead_code)]

fn two_sum(nums: &[i64], target: i64) -> Vec<usize> {
    use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_panics_on_no_solution() {
        two_sum(&[2, 7, 11, 15], 50);
    }

    #[test]
    fn returns_correct_indices() {
        let result = two_sum(&[2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);

        let result = two_sum(&[3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);

        let result = two_sum(&[3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }
}
