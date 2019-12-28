use crate::advent07::intcode::IntCode;
use crate::advent07::intio::IoList;
use std::cmp::max;
use std::collections::HashSet;
use crate::advent07::model::ProgramState;

struct Amplifier {
    intcode: IntCode,
    input_list: IoList,
}

impl<'a> Amplifier {
    fn create(intcode: IntCode, phase_setting: i32) -> Amplifier {
        Amplifier {
            intcode,
            input_list: {
                let mut list = IoList::create_empty();
                list.push(phase_setting);
                list
            },
        }
    }
    fn exec(&mut self, next_amplifier: &mut Amplifier) {
        println!("input_list: {:#?}", self.input_list);
        println!("intcode, before_exec: {:#?}", self.intcode);

        self.intcode.exec(&mut self.input_list.create_input_fn(),
                          &mut next_amplifier.input_list.create_output_fn());
        println!("intcode, after_exec: {:#?}", self.intcode);

//        println!("{:?}", self.intcode.program_state)
    }
    fn is_halted(&self) -> bool {
        match self.intcode.program_state {
            ProgramState::Halted => true,
            _ => false
        }
    }
}

pub fn execute_amplifiers_looped(code: &Vec<i32>, phase_settings: Vec<i32>) -> i32 {
    let mut amp_a = Amplifier::create(IntCode::create(code), phase_settings[0]);
    let mut amp_b = Amplifier::create(IntCode::create(code), phase_settings[1]);
    let mut amp_c = Amplifier::create(IntCode::create(code), phase_settings[2]);
    let mut amp_d = Amplifier::create(IntCode::create(code), phase_settings[3]);
    let mut amp_e = Amplifier::create(IntCode::create(code), phase_settings[4]);
    amp_a.input_list.push(0);
    while !amp_a.is_halted() {
        println!("amp_a");
        amp_a.exec(&mut amp_b);
        println!("amp_b");
        amp_b.exec(&mut amp_c);
        println!("amp_c");
        amp_c.exec(&mut amp_d);
        println!("amp_d");
        amp_d.exec(&mut amp_e);
        println!("amp_e");
        amp_e.exec(&mut amp_a);
    }
    amp_a.input_list.pop()
}


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

fn unique(a: i32, b: i32, c: i32, d: i32, e: i32) -> bool {
    let mut h: HashSet<i32> = HashSet::new();
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
        let code = vec!(3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0);
        let phase_settings = vec!(4, 3, 2, 1, 0);
        assert_eq!(execute_amplifiers(&code, phase_settings), 43210)
    }

    #[test]
    fn test_highest_signal() {
        let code =
            vec!(3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33,
                 1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0);
        assert_eq!(highest_signal(&code), 65210)
    }

    #[test]
    fn test_execute_amplifiers_looped() {
        let code =
            vec!(3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26,
                 27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5);

        assert_eq!(execute_amplifiers_looped(&code, vec!(9, 8, 7, 6, 5)), 139629729);

    }
}