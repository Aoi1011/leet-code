// #24

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_ways(n: i32, k: i32) -> i32 {
        if n == 0 || k == 0 {
            return 0;
        }

        if n == 1 {
            return k;
        }

        if k == 1 {
            return 1;
        }

        let mut same = k;
        let mut diff = k * (k - 1);

        for _ in 2..n {
            let t_same = same;
            let t_diff = diff;
            same = t_diff;
            diff = (t_same + t_diff) * (k - 1);
        }

        same + diff
    }
}

#[cfg(test)]
mod tests {
    use crate::paint_fence::*;
    #[test]
    fn test_num_ways() {
        let result = Solution::num_ways(1, 2);
        println!("Result: {:?}", result);
    }
}
