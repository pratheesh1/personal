#![allow(dead_code)]

pub fn length_of_longest_substring(s: String) -> i32 {
    // a b c d b c d e f <- input
    // a b c d           <- len = 4
    // 	c d b            <- len = 3
    // 	  d b c          <- len = 3
    // 	    b c d e f    <- len = 5
    // => length_of_longest_substring = 5

    use std::collections::HashMap;

    let mut max_len = 0;
    let mut last_index_map = HashMap::<char, usize>::new();

    for (i, char) in s.chars().enumerate() {
        if let Some(last_index) = last_index_map.get(&char) {
            let substr = String::from(s.get(last_index.clone() + 1..).unwrap());
            let substr = length_of_longest_substring(substr);
            if substr > max_len {
                max_len = substr;
            }
            break;
        } else {
            max_len += 1;
            last_index_map.insert(char, i);
        }
    }

    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_correct_len_of_longest_substring() {
        assert_eq!(length_of_longest_substring(String::new()), 0);
        assert_eq!(length_of_longest_substring(String::from(" ")), 1);

        assert_eq!(length_of_longest_substring("a".to_string()), 1);
        assert_eq!(length_of_longest_substring("aa".to_string()), 1);
        assert_eq!(length_of_longest_substring("au".to_string()), 2);
        assert_eq!(length_of_longest_substring("au".to_string()), 2);
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("thankfully ok".to_string()), 8);
    }
}
