use crate::InterpretError;

#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum OpCode {
  CONSTANT,
  NEGATE,
  RETURN,
  ADD,
  SUBTRACT,
  MULTIPLY,
  DIVIDE,
}

impl TryFrom<u8> for OpCode {
    type Error = InterpretError ;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::CONSTANT),
            1 => Ok(OpCode::NEGATE),
            2 => Ok(OpCode::RETURN),
            3 => Ok(OpCode::ADD),
            4 => Ok(OpCode::SUBTRACT),
            5 => Ok(OpCode::MULTIPLY),
            6 => Ok(OpCode::DIVIDE),
            _ => Err(InterpretError::CompileError)
          }
    }
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        value as u8
    }
}