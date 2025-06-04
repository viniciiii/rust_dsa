
struct  Solution{

}

impl Solution {
    pub fn is_palindrome(s: String) -> bool  {
        let sv = s.chars().filter(|b| b.is_alphanumeric()).map(|c| c.to_ascii_lowercase());
        return sv.clone().eq(sv.rev());
    }
}
pub fn run () {
    let s = String::from("race a car");
    println!("{:?}", Solution::is_palindrome(s));
}