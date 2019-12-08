use crate::advent02::intcode::Operation::{Addition, Multiplication, Halt};

pub fn exec(code: &mut Vec<i32>) {
    let mut program_counter: i32 = 0;
    let mut next_program_action = NextProgramAction::Continue;
    while let NextProgramAction::Continue = next_program_action {
        next_program_action = parse_and_execute_single_operation(code, program_counter);
        program_counter += 4;
    }
}

fn parse_and_execute_single_operation(code: &mut Vec<i32>, program_counter: i32) -> NextProgramAction {
    let operation = Operation::parse_at_program_counter(code, program_counter);
    operation.execute(code)
}

enum Operation {
    Addition { value_1_index: i32, value_2_index: i32, result_index: i32 },
    Multiplication { value_1_index: i32, value_2_index: i32, result_index: i32 },
    Halt,
    Error,
}

enum NextProgramAction {
    Continue,
    Halt,
    Fail,
}

impl Operation {
    fn parse_at_program_counter(code: &Vec<i32>, program_counter: i32) -> Operation {
        let pcu: usize = program_counter as usize;
        match code[pcu] {
            1 => Addition {
                value_1_index: code[pcu + 1],
                value_2_index: code[pcu + 2],
                result_index: code[pcu + 3],
            },
            2 => Multiplication {
                value_1_index: code[pcu + 1],
                value_2_index: code[pcu + 2],
                result_index: code[pcu + 3],
            },
            99 => Halt,
            _ => Operation::Error
        }
    }

    fn execute(&self, code: &mut Vec<i32>) -> NextProgramAction {
        match self {
            Operation::Addition { value_1_index, value_2_index, result_index } => {
                code[*result_index as usize] = code[*value_1_index as usize] + code[*value_2_index as usize];
                NextProgramAction::Continue
            }
            Operation::Multiplication { value_1_index, value_2_index, result_index } => {
                code[*result_index as usize] = code[*value_1_index as usize] * code[*value_2_index as usize];
                NextProgramAction::Continue
            }
            Operation::Halt => NextProgramAction::Halt,
            Operation::Error => NextProgramAction::Fail
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_operation() {
        let mut immutable_code = vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50);
        let code = immutable_code.as_mut();
        let expected_result : Vec<i32> = vec!(1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50);
        let first_command_result: NextProgramAction =
            parse_and_execute_single_operation(code, 0);
        assert_eq!(code, &expected_result)
    }

    #[test]
    fn test_complete_execution() {
        let mut immutable_code = vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50);
        let code = immutable_code.as_mut();
        let expected_result : Vec<i32> = vec!(3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50);
        exec(code);
        assert_eq!(code, &expected_result)
    }
}


