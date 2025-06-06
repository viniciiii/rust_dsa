// use std::{mem::swap, usize, vec};

use std::collections::HashMap;

#[derive(Debug)]
struct Solution {

}

//bruteforce approach o(n2) time complexity

// impl Solution {
//     pub fn first_uniq_char(s: String) -> i32  {
//         let chars: Vec<char> = s.chars().collect();
//         // println!("{:?}", chars);
//         for i in 0..chars.len() {
//             let mut count = 0;
//             for j in 0..chars.len() {
//                 if chars[i] == chars[j] {
//                     count = count + 1;
//                 }
//             }
//             if count == 1 {
//                 return i as i32;
//             }
//         }
//         -1
//     }
// }


// approach 2 with less time complexity o(n) with 12 ms
// impl Solution {
//     pub fn first_uniq_char(s: String) -> i32 {
//         let mut map = HashMap::new();
//         // let h: Vec<char> = s.chars().collect();
//         for c in s.chars() {
//             *map.entry(c).or_insert(0) += 1;
//         }
//     // println!("{:?}", map);
//     for (i, c) in s.chars().enumerate() {
//         if map[&c] == 1 {
//             return i as i32;
//         }
//     }
//     -1
        
//     }
// }

// approach 3 time complexity o(n) with fastest 0ms
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut v = vec![0; 256];
        // let h: Vec<char> = s.chars().collect();
        for c in s.as_bytes() {
            v[*c as usize] += 1;
        }
    // println!("{:?}", v);
    for (i, c) in s.as_bytes().iter().enumerate() {
        if v[*c as usize] == 1 {
            return i as i32;
        }
    }
    -1 
    }
}
pub fn run () {
    let s = String::from("leetcode");
    println!("{:?}", Solution::first_uniq_char(s));
}