use crate::advent07::intcode::IntCode;
use crate::advent07::intio::IoList;
use std::cmp::max;
use std::collections::HashSet;

//pub fn execute_amplifiers_looped(code: &Vec<i32>, phase_settings: Vec<i32>) -> i32 {
//    let mut input_a = IoList::create_empty();
//    let mut input_b = IoList::create_empty();
//    let mut input_c = IoList::create_empty();
//    let mut input_d = IoList::create_empty();
//    let mut input_e = IoList::create_empty();
//}

pub fn execute_amplifiers(code: &Vec<i32>, phase_settings: Vec<i32>) -> i32 {
    let mut amplified_value = 0;
    for phase_setting in phase_settings {
        let mut intcode = IntCode::create(code);
        amplified_value = execute_amplifier(&mut intcode, phase_setting, amplified_value);
    }
    amplified_value
}

fn execute_amplifier(intcode: &mut IntCode, phase_setting: i32, amplifier_input: i32) -> i32 {
    let mut il = IoList::create_empty();
    il.push(phase_setting);
    il.push(amplifier_input);
    let mut ol = IoList::create_empty();
    intcode.exec(&mut il.create_input_fn(), &mut ol.create_output_fn());
    ol.pop()
}

pub fn highest_signal(code: &Vec<i32>) -> i32 {
    let mut current_highest_signal = -1;
    for a in 0..5 {
        for b in 0..5 {
            for c in 0..5 {
                for d in 0..5 {
                    for e in 0..5 {
                        if unique(a, b, c, d, e) {
                            current_highest_signal =
                                max(current_highest_signal,
                                    execute_amplifiers(code, vec!(a, b, c, d, e)));
                        }
                    }
                }
            }
        }
    }
    current_highest_signal
}
fn unique(a:i32,b:i32,c:i32,d:i32,e:i32) -> bool {
    let mut h : HashSet<i32> = HashSet::new();
    h.insert(a);
    h.insert(b);
    h.insert(c);
    h.insert(d);
    h.insert(e);
    h.len() == 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_amplifiers() {
        let code = vec!(3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0);
        let phase_settings = vec!(4,3,2,1,0);
        assert_eq!(execute_amplifiers(&code, phase_settings), 43210)
    }

    #[test]
    fn test_highest_signal() {
        let code =
            vec!(3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                 1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0);
        assert_eq!(highest_signal(&code), 65210)
    }
}