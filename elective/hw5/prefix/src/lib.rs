#![forbid(unsafe_code)]

use std::cmp::min;

pub fn longest_common_prefix(strs: Vec<&str>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let mut ans: &[char] = &strs[0].chars().collect::<Vec<char>>();
    for string in &strs[..] {
        let min_len = min(ans.len(), string.chars().count());
        if min_len == 0 {
            return String::new();
        }

        let mut index = 0;
        for char in string.chars() {
            if char != ans[index] {
                ans = &ans[0..index];
                break;
            }

            if index + 1 >= min_len {
                ans = &ans[0..=index];
                break;
            }
            index += 1;
        }
    }
    String::from_iter(ans)
}
