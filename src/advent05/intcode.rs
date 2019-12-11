use crate::advent05::model::Operation;
use crate::advent05::model::NextProgramAction;

pub fn exec(code: &mut Vec<i32>) {
    let mut program_counter: i32 = 0;
    let mut next_program_action = NextProgramAction::Continue;

    while let NextProgramAction::Continue = next_program_action {
        let (npa, program_counter_increase) =
            parse_and_execute_single_operation(code, program_counter);
        program_counter += program_counter_increase;
        next_program_action = npa;
    }
}

fn parse_and_execute_single_operation(code: &mut Vec<i32>, program_counter: i32) -> (NextProgramAction, i32) {
    let operation = Operation::parse_at_program_counter(code, program_counter);
    operation.execute(code)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_operation() {
        let mut immutable_code = vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50);
        let code = immutable_code.as_mut();
        let expected_result : Vec<i32> = vec!(1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50);
//        let (first_command_result, _) =
            parse_and_execute_single_operation(code, 0);
        assert_eq!(code, &expected_result)
    }

    #[test]
    fn test_complete_execution() {
        let mut immutable_code = vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50);
        let code = immutable_code.as_mut();
        let expected_result: Vec<i32> = vec!(3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50);
        exec(code);
        assert_eq!(code, &expected_result)
    }
}


