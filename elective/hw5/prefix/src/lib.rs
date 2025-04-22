#![forbid(unsafe_code)]

use std::cmp::min;

pub fn longest_common_prefix(strs: Vec<&str>) -> String {
    let mut ans: &[char] = &strs[0].chars().collect::<Vec<char>>();
    for string in &strs[..] {
        let min_len = min(ans.len(), string.len());
        if min_len == 0 {
            return String::new();
        }

        for (index, char) in string.char_indices() {
            if char != ans[index] {
                ans = &ans[0..index];
                break;
            }

            if index + 1 >= min_len {
                ans = &ans[0..=index];
                break;
            }
        }
    }
    String::from_iter(ans)
}
