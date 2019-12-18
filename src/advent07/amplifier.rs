use crate::advent07::intcode::exec;
use crate::advent07::intio::{InputList, OutputList};

pub fn execute_amplifier(code: &mut Vec<i32>, phase_setting: i32, amplifier_input: i32) -> i32 {
    let mut il = InputList::create_empty();
    il.push(phase_setting);
    il.push(amplifier_input);
    let mut ol = OutputList::create_empty();
    exec(code, &mut il.create_input_fn(), &mut ol.create_output_fn());
    ol.pop()
}