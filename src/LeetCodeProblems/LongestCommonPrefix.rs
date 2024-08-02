pub struct Solution{

}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let mut strs = strs; // Make strs mutable so we can sort it
        strs.sort();

        let first = strs.first().unwrap();
        let last = strs.last().unwrap();

        let mut i = 0;
        while i < first.len() && first.as_bytes()[i] == last.as_bytes()[i] {
            i += 1;
        }

        first[..i].to_string()
    }
}

pub struct Driver;

impl Driver {
    pub fn run_tests() {
        let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
        let expected_common_str = "fl";
        assert_eq!(Solution::longest_common_prefix(strs), expected_common_str);
    }
}

