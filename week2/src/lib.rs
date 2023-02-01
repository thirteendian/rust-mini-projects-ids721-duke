//build a function Solution that takes a string
//and returns the longest palindromic substring in the string
pub fn longest_palindrome(s: String) -> String {
    let mut longest = String::new();
    let mut longest_len = 0;
    let mut i = 0;
    while i < s.len() {
        let mut j = i + 1;
        while j <= s.len() {
            let sub = &s[i..j];
            if is_palindrome(sub) && sub.len() > longest_len {
                longest = sub.to_string();
                longest_len = sub.len();
            }
            j += 1;
        }
        i += 1;
    }
    longest
}

//build a function is_palindrome that takes a string
//and returns true if the string is a palindrome
fn is_palindrome(s: &str) -> bool {
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if s.chars().nth(i) != s.chars().nth(j) {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}
