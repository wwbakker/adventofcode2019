
fn are_digits_adjacent(s : &[u8]) -> bool {
    s[0] == s[1] || s[1] == s[2] || s[2] == s[3] || s[3] == s[4] || s[4] == s[5]
}

fn ltr_never_decrease(s : &[u8]) -> bool {
    s[0] <= s[1] && s[1] <= s[2] && s[2] <= s[3] && s[3] <= s[4] && s[4] <= s[5]
}

pub fn start_a() {
    let mut s : String;
    let mut num : i32 = 0;
    for i in 248345..746315 {
        s = i.to_string();
        if are_digits_adjacent(s.as_bytes()) && ltr_never_decrease(s.as_bytes()) {
            num += 1;
        }
    }
    println!("{} different passwords", num)
}
