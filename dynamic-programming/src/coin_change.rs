// #34

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // if coins.len() <= 1 {
        //     if amount > coins[0] {
        //         return -1;
        //     } else if coins[0] > amount {
        //         return 0;
        //     } else {
        //         return coins[0]
        //     }
        // }

        let mut clone_coins = coins.clone();
        let mut ret = 0;
        let mut tem_amount = amount;

        clone_coins.sort();
        clone_coins.reverse();

        for coin in clone_coins.iter() {
            if tem_amount >= *coin {
                ret += tem_amount / coin;
                tem_amount = tem_amount % coin;
            }
        }

        if tem_amount != 0 {
            return -1;
        }

        ret
    }

    #[allow(dead_code)]
    pub fn coin_change_1(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![None; amount + 1];
        dp[0] = Some(0);

        for i in 1..=amount {
            dp[i] = coins
                .iter()
                .filter_map(|&j| {
                    let j = j as usize;
                    if j <= i {
                        dp[i - j].map(|n| n + 1)
                    } else {
                        None
                    }
                })
                .min();
        }

        dp[amount].unwrap_or(-1)
    }

    #[allow(dead_code)]
    pub fn coin_change_02(coins: Vec<i32>, amount: i32) -> i32 {
        // Go from bottom to top
        let mut dp = vec![amount + 1; (amount + 1) as usize];
        dp[0] = 0;
        for i in 1..=amount {
            for &c in coins.iter() {
                if i >= c {
                    dp[i as usize] = i32::min(dp[i as usize], 1 + dp[(i - c) as usize]);
                }
            }
        }

        let last = *dp.last().unwrap();
        if last > amount {
            return -1;
        } else {
            return last;
        }
    }

    #[allow(dead_code)]
    pub fn coin_change_03(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; (amount + 1) as usize];
        dp[0] = 0;

        for index in 1..=amount {
            for coin in coins.iter() {
                if index - coin >= 0 {
                    dp[index as usize] =
                        i32::min(dp[index as usize], 1 + dp[(index - coin) as usize]);
                }
            }
        }

        if dp[amount as usize] != amount + 1 {
            return dp[amount as usize];
        } else {
            return -1;
        }
    }
}

#[test]
fn test_coin_change() {
    let coins = vec![1, 2, 5];
    let amount = 11;

    let result = Solution::coin_change_03(coins, amount);
    assert_eq!(result, 3);
}

#[test]
fn test_coin_change_02() {
    let coins = vec![2];
    let amount = 3;

    let result = Solution::coin_change_03(coins, amount);
    assert_eq!(result, -1);
}

#[test]
fn test_coin_change_04() {
    let coins = vec![1];
    let amount = 2;

    let result = Solution::coin_change_03(coins, amount);
    assert_eq!(result, 2);

    println!("{:?}", 11 % 2);
}
