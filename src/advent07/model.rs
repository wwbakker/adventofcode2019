#[derive(PartialEq,Debug,Clone)]
pub(crate) enum ParameterMode {
    Immediate,
    Position
}

impl ParameterMode {

    fn parse_from_char(c : char) -> ParameterMode {
        if c == '0' {ParameterMode::Position}
        else if c == '1' {ParameterMode::Immediate}
        else {panic!("Unexpected parameter mode.")}
    }
}

pub(crate) enum Value {
    Immediate {value : i32},
    Position {index : i32}
}
impl Value {
    fn create(v : i32, parameter_mode: &ParameterMode) -> Value {
        match parameter_mode {
            ParameterMode::Immediate => Value::Immediate { value: v },
            ParameterMode::Position => Value::Position { index: v }
        }
    }

    fn read(&self, code : &Vec<i32>) -> i32 {
        match self {
            Value::Position { index} => code[*index as usize],
            Value::Immediate { value} => *value
        }
    }

    fn write(&self, code: &mut Vec<i32>, value_to_write : i32) -> () {
        match self {
            Value::Position { index} => code[*index as usize] = value_to_write,
            Value::Immediate { value: _ } => panic!("writing to immediate does not make sense.")
        }
    }
}
pub(crate) enum NextProgramAction {
    Continue,
    Halt,
}

pub(crate) enum Operation {
    Addition { value_1 : Value, value_2: Value, result: Value },
    Multiplication { value_1: Value, value_2: Value, result: Value },
    Input { destination : Value },
    Output { source : Value },
    JumpIfTrue { value : Value, jump_address: Value },
    JumpIfFalse { value : Value, jump_address: Value },
    LessThan { value_1: Value, value_2: Value, result: Value },
    Equals { value_1: Value, value_2: Value, result: Value },
    Halt,
}

impl Operation {

    fn parameters_per_opcode(opcode : i32) -> i32 {
        match opcode {
            1 => 3,
            2 => 3,
            3 => 1,
            4 => 1,
            5 => 2,
            6 => 2,
            7 => 3,
            8 => 3,
            99 => 0,
            _ => panic!("unknown upcode")
        }
    }

    fn parse_opcode_and_parameter_modes(code : i32) -> (i32, Vec<ParameterMode>) {
        let s : String = code.to_string();
        let opcode_1 = s.chars().rev().nth(1).unwrap_or('0');
        let opcode_2 = s.chars().rev().nth(0).unwrap();
        let opcode : i32 = format!("{}{}",
                                   opcode_1,
                                   opcode_2).parse().unwrap();


        let num_of_parameters = Self::parameters_per_opcode(opcode);

        let parameter_modes =
            (0..num_of_parameters)
                .map(|i| s.chars().rev().nth((i + 2) as usize).map(ParameterMode::parse_from_char).unwrap_or(ParameterMode::Position))
                .collect();

        (opcode, parameter_modes)
    }



    pub(crate) fn parse_at_program_counter(code: &Vec<i32>, program_counter: i32) -> Operation {
        let pcu: usize = program_counter as usize;
        let (opcode, parameter_modes) = Self::parse_opcode_and_parameter_modes(code[pcu]);
        match opcode {
            1 => Operation::Addition {
                value_1: Value::create(code[pcu + 1], &parameter_modes[0]),
                value_2: Value::create(code[pcu + 2], &parameter_modes[1]),
                result: Value::create(code[pcu + 3], &parameter_modes[2]),
            },
            2 => Operation::Multiplication {
                value_1: Value::create(code[pcu + 1], &parameter_modes[0]),
                value_2: Value::create(code[pcu + 2], &parameter_modes[1]),
                result: Value::create(code[pcu + 3], &parameter_modes[2]),
            },
            3 => Operation::Input {
                destination: Value::create( code[pcu + 1], &parameter_modes[0])
            },
            4 => Operation::Output {
                source: Value::create( code[pcu + 1], &parameter_modes[0])
            },
            5 => Operation::JumpIfTrue {
                value: Value::create( code[pcu + 1], &parameter_modes[0]),
                jump_address: Value::create( code[pcu + 2], &parameter_modes[1])
            },
            6 => Operation::JumpIfFalse {
                value: Value::create( code[pcu + 1], &parameter_modes[0]),
                jump_address: Value::create( code[pcu + 2], &parameter_modes[1])
            },
            7 => Operation::LessThan {
                value_1: Value::create(code[pcu + 1], &parameter_modes[0]),
                value_2: Value::create(code[pcu + 2], &parameter_modes[1]),
                result: Value::create(code[pcu + 3], &parameter_modes[2]),
            },
            8 => Operation::Equals {
                value_1: Value::create(code[pcu + 1], &parameter_modes[0]),
                value_2: Value::create(code[pcu + 2], &parameter_modes[1]),
                result: Value::create(code[pcu + 3], &parameter_modes[2]),
            },
            99 => Operation::Halt,
            _ => panic!("unknown opcode")
        }
    }

    pub(crate) fn execute(&self,
                          code: &mut Vec<i32>,
                          pcu : i32,
                          read_int_function: &dyn Fn() -> i32,
                          output_function: &mut dyn FnMut(i32) -> ()) -> (NextProgramAction, i32) {
        match self {
            Operation::Addition { value_1, value_2, result } => {
                result.write(code, value_1.read(code) + value_2.read(code));
                (NextProgramAction::Continue, pcu + 4)
            },
            Operation::Multiplication { value_1, value_2, result } => {
                result.write(code, value_1.read(code) * value_2.read(code));
                (NextProgramAction::Continue, pcu + 4)
            },
            Operation::Input { destination} => {
                destination.write(code, read_int_function());
                (NextProgramAction::Continue, pcu + 2)
            },
            Operation::Output { source} => {
                output_function(source.read(code));
                (NextProgramAction::Continue, pcu + 2)
            },
            Operation::JumpIfTrue { value, jump_address} => {
                let next_instruction : i32 =
                    if value.read(code) != 0
                        { jump_address.read(code) }
                    else
                        { pcu + 3 };
                (NextProgramAction::Continue, next_instruction)
            },
            Operation::JumpIfFalse { value, jump_address} => {
                let next_instruction : i32 =
                    if value.read(code) == 0
                    { jump_address.read(code) }
                    else
                    { pcu + 3 };
                (NextProgramAction::Continue, next_instruction)
            },
            Operation::LessThan { value_1, value_2, result} => {
                result.write(code, if value_1.read(code) < value_2.read(code) {1} else {0} );
                (NextProgramAction::Continue, pcu + 4)
            }
            Operation::Equals { value_1, value_2, result} => {
                result.write(code, if value_1.read(code) == value_2.read(code) {1} else {0} );
                (NextProgramAction::Continue, pcu + 4)
            }
            Operation::Halt => (NextProgramAction::Halt, 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_opcode_and_parameter_modes() {
        let (opcode, parameter_modes) = Operation::parse_opcode_and_parameter_modes(1002);
        assert_eq!(opcode, 2);
        assert_eq!(parameter_modes, vec!(ParameterMode::Position, ParameterMode::Immediate, ParameterMode::Position));
    }
}


