use std::str::FromStr;

use helpers::parse_error::ParseError;

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    ACC,
    JMP,
    NOP,
}

#[derive(Debug, Copy, Clone)]
pub struct InputRecord {
    pub op: Operation,
    pub value: i32,
}

impl FromStr for InputRecord {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find(' ') {
            Some(pos) => {
                let op: Operation = match &s[..pos] {
                    "acc" => Ok(Operation::ACC),
                    "jmp" => Ok(Operation::JMP),
                    "nop" => Ok(Operation::NOP),
                    _ => Err(ParseError(format!("Invalid input line: '{}'", s))),
                }?;
                let value: i32 = s[pos + 1..].parse::<i32>()?;

                Ok(InputRecord { op, value })
            }
            None => Err(ParseError(format!("Invalid input line: '{}'", s))),
        }
    }
}
