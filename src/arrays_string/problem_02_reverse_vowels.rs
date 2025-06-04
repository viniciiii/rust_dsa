use std::{mem::swap, usize, vec};

#[derive(Debug)]
struct Solution {

}

//approach 1 
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        fn is_vowel(c: char) -> bool {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => true,
                _ => false
            }
        }

        let (mut left, mut right) = (0, s.len() - 1);
        let mut cs: Vec<char> = s.chars().collect();

        while left < right {
            if !is_vowel(cs[left].to_ascii_lowercase()) {
                left += 1;
                continue
            }
            if !is_vowel(cs[right].to_ascii_lowercase()) {
                right -= 1;
                continue
            }

            cs.swap(left, right);
            left += 1;
            right -= 1;
        }
        cs.iter().collect()
    }
}


// approach 2nd(effiecient as we are working directly with bytes)
// impl Solution {
//     pub fn reverse_vowels(mut s: String) -> String {
//         // From the description, `s` is guaranteed to be ASCII, so this is fine
//         let mut bytes = unsafe { s.as_bytes_mut() };

//         let mut iter = bytes.iter_mut();
//         while let (Some(left), Some(right)) = (
//             iter.find(|c| is_vowel(c)),
//             iter.rfind(|c| is_vowel(c))
//         ) {
//             std::mem::swap(left, right);
//         }

//         s
//     }
// }

// fn is_vowel(c: &u8) -> bool {
//     matches!(c.to_ascii_lowercase(), b'a' | b'e' | b'i' | b'o' | b'u')
// }

pub fn run() {
    let s = String::from("IceCreAm");
    let ans = Solution::reverse_vowels(s);
    println!("reversed vowels string is : {:?}", ans);
}