#[allow(dead_code)]
struct Solution;

use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let sum = target - *num;
            if map.contains_key(&sum) {
                return vec![map[&sum], i as i32];
            }
            map.insert(*num, i as i32);
        }
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn base_case() {
        assert_eq!(
            Solution::two_sum(vec![2, 7, 11, 15], 9),
            vec![0, 1]
        )
    }
    
    #[test]
    fn case1() {
        assert_eq!(
            Solution::two_sum(vec![2, 7, 11, 15], 17),
            vec![0, 3]
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::two_sum(vec![2, 15], 17),
            vec![0, 1]
        )
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::two_sum(vec![20, -15], 5),
            vec![0, 1]
        )
    }
}