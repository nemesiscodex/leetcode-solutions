#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // XOR
        nums.iter().fold(0, |a, b| a ^ *b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base_case1() {
        assert_eq!(
            Solution::single_number(vec![2, 2, 1]),
            1
        );
    }

    #[test]
    fn base_case2() {
        assert_eq!(
            Solution::single_number(vec![4, 1, 2, 1, 2]),
            4
        );
    }


    #[test]
    fn case1() {
        assert_eq!(
            Solution::single_number(vec![2, 1, 4, 1, 2]), 
            4
        );
    }



    #[test]
    fn case2() {
        assert_eq!(
            Solution::single_number(vec![2, 1, 1, 2, 4]), 
            4
        );
    }


    #[test]
    fn case3() {
        assert_eq!(
            Solution::single_number(vec![2, 1, 2]), 
            1
        );
    }
}
