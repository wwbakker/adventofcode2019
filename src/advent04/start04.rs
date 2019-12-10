
fn are_digits_adjacent(s : &[u8]) -> bool {
    s[0] == s[1] || s[1] == s[2] || s[2] == s[3] || s[3] == s[4] || s[4] == s[5]
}
fn are_digits_adjacent_2_not_3(s : &[u8]) -> bool {
    are_digits_adjacent_2_not_3_from_index(s, 0) ||
        are_digits_adjacent_2_not_3_from_index(s, 1) ||
        are_digits_adjacent_2_not_3_from_index(s, 2) ||
        are_digits_adjacent_2_not_3_from_index(s, 3) ||
        are_digits_adjacent_2_not_3_from_index(s, 4)
}

fn are_digits_adjacent_2_not_3_from_index(s : &[u8], i : usize) -> bool {
    if i == 0 {
        (s[0] == s[1] && s[1] != s[2])
    } else if i == 4 {
        (s[3] != s[4] && s[4] == s[5])
    } else {
        (s[i-1] != s[i] && s[i] == s[i + 1] && s[i + 1] != s[i + 2])
    }

}

fn ltr_never_decrease(s : &[u8]) -> bool {
    s[0] <= s[1] && s[1] <= s[2] && s[2] <= s[3] && s[3] <= s[4] && s[4] <= s[5]
}

pub fn start_a() {
    let mut s : String;
    let mut num : i32 = 0;
    for i in 248345..746315 {
        s = i.to_string();
        if are_digits_adjacent(s.as_bytes()) &&
            ltr_never_decrease(s.as_bytes()) {
            num += 1;
        }
    }
    println!("{} different passwords (a)", num)
}

pub fn start_b() {
    let mut s : String;
    let mut num : i32 = 0;
    for i in 248345..746315 {
        s = i.to_string();
        if are_digits_adjacent_2_not_3(s.as_bytes()) &&
            ltr_never_decrease(s.as_bytes()) {
            num += 1;
        }
    }
    println!("{} different passwords(b)", num)
}