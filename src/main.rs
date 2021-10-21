fn main() {
    let nums: Vec<i32> = vec![1, 2];
    println!("count={:?}", Solution::min_moves(nums));
}

struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        if nums.len() > 1 {
            let mut nums = nums;
            nums.sort();
            let len = nums.len() - 1;
            let mut i = len;
            loop {
                if i == 0 {
                    break;
                }
                i -= 1;
                if nums[nums.len() - 1] == nums[i] {

                } else {
                    let add = nums[nums.len() - 1] - nums[i];
                    for (index, v) in nums.iter_mut().enumerate() {
                        if index < len - 1 {
                            *v += add;
                        }
                    }
                    println!("{:?}", nums);

                    count += add;
                }
            }
        }

        count
    }
}