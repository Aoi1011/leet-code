/*
#32
You are given an integer array prices where prices[i] is the price of a given stock on the ith day.

On each day, you may decide to buy and/or sell the stock.
You can only hold at most one share of the stock at any time. However, you can buy it then immediately sell it on the same day.

Find and return the maximum profit you can achieve.

Input: prices = [7,1,5,3,6,4]
Output: 7
Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
Total profit is 4 + 3 = 7.

*/

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // prices.as_slice().windows(2).fold()
        prices.windows(2).fold(
            0,
            |acc, x| {
                if x[0] < x[1] {
                    x[1] - x[0] + acc
                } else {
                    acc
                }
            },
        )
    }

    pub fn max_profit_1(prices: Vec<i32>) -> i32 {
        prices.iter().zip(prices[1..].iter()).fold(0, |acc, (a, b)| {
            if a < b {
                b - a + acc
            } else {
                acc
            }
        })
    }
}

#[test]
fn test_max_profit() {
    let input = vec![7, 1, 5, 3, 6, 4];
    let ans = Solution::max_profit_1(input);
    assert_eq!(ans, 7);
}

#[test]
fn test_max_profit_2() {
    let input = vec![1, 2, 3, 4, 5];
    let ans = Solution::max_profit_1(input);
    assert_eq!(ans, 4);
}

#[test]
fn test_max_profit_3() {
    let input = vec![7, 6, 4, 3, 1];
    let ans = Solution::max_profit_1(input);
    assert_eq!(ans, 0);
}