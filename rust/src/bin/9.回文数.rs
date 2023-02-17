/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-02-17 09:27:04
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-02-17 09:34:09
 * @FilePath: /rust/src/bin/9.回文数.rs
 * @Description: 
 */
/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x % 10 == 0 && x != 0 {
            return false
        }

        let mut reverse_num = 0;
        while reverse_num < x {
            reverse_num = reverse_num * 10 + x % 10;
            x /= 10
        }

        reverse_num == x || reverse_num / 10 == x
    }
}
// @lc code=end

