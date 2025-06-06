use std::collections::HashMap;


struct  Solution{

}

// approach 1 with more complexity and 2ms 
// impl Solution {
//     pub fn frequency_sort(s: String) -> String {
//         let mut map = HashMap::new();
//         for c in s.chars() {
//             *map.entry(c).or_insert(0) += 1;
//         }
        
//         let mut counts: Vec<_> = map.iter().collect();

//         counts.sort_by(|(_, a), (_, b)| b.cmp(a));

//         let mut s = String::with_capacity(s.len());

//         for (k, v) in counts {
//             for _ in 0..*v {
//                 s.push(*k);
//             }
//         }

//         return s;
//     }
// }


//not that much efficient but mostly worked with iterator functions so good to know
use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let counter = s.chars().fold(HashMap::new(), |mut count, ch| {
            *count.entry(ch).or_insert(0) += 1;
            count
        });    
        let mut counter_vec: Vec<(char, i32)> = counter.iter().map(|(&ch, &freq)| { (ch, freq) }).collect();
        counter_vec.sort_by(|(ch1, freq1), (ch2, freq2)| freq2.cmp(freq1));
        counter_vec.iter().map(|(ch, freq)| { ch.to_string().repeat(*freq as usize) }).collect::<Vec<_>>().join("")
    }
}

pub fn run () {
    let s = String::from("cccaaa");
    println!("{:?}", Solution::frequency_sort(s));
}