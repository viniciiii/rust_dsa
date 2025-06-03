#[derive(Debug)]
struct Solution {

}
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let (mut start, mut end) = (0, s.len() - 1);

        while start < end {
            s.swap(start, end);
            start = start + 1;
            end = end - 1;
        } 
    }
}

pub fn run() {
    let mut input: Vec<char> = vec!['h', 'e', 'l', 'l', '0'];
    Solution::reverse_string(&mut input);
    println!("reversed string is : {:?}", input);
}