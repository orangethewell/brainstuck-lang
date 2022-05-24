
pub mod InstConsts {
    pub const INC_PTR: char = '>';
    pub const INC_BYTE: char = '+';
    pub const DEC_BYTE: char = '-';
    pub const WRITE_BYTE: char = '.';
    pub const DEC_PTR: char = '<';
    pub const READ_BYTE: char = ',';
    pub const LOOP_END: char = ']';
    pub const LOOP_START: char = '[';
    pub const REG_UP: char = '^';
    pub const REG_DOWN: char = 'v';
    pub const ENV_OPEN: char = '(';
    pub const ENV_CLOSE: char = ')';
    pub const COPY_FN: char = '&';
    pub const IF_STATEM: char = '?';
}

pub const INSTS: &[char; 14] = &[
    InstConsts::INC_PTR,
    InstConsts::DEC_PTR,
    InstConsts::INC_BYTE,
    InstConsts::DEC_BYTE,
    InstConsts::WRITE_BYTE,
    InstConsts::READ_BYTE,
    InstConsts::LOOP_START,
    InstConsts::LOOP_END,
    InstConsts::REG_UP,
    InstConsts::REG_DOWN,
    InstConsts::ENV_OPEN,
    InstConsts::ENV_CLOSE,
    InstConsts::COPY_FN,
    InstConsts::IF_STATEM
];


#[derive(Debug, PartialEq, Eq)]
pub enum InstructionKind {
    IncPtr,
    DecPtr,
    IncByte,
    DecByte,
    WriteByte,
    ReadByte,
    LoopStart { end_index: usize },
    LoopEnd { start_index: usize },
    RegUp,
    RegDown,
    EnvOpen,
    EnvClose,
    CopyFn,
    IfStatem
}

impl InstructionKind {
    pub fn add_jump_index(&mut self, jump_index: usize) {
        match self {
            InstructionKind::LoopStart { end_index } => *end_index = jump_index,
            InstructionKind::LoopEnd { start_index } => *start_index = jump_index,
            _ => panic!("trying to set jmp_idx {} on {:?}", jump_index, self)
        }
    }
}

/// A simple node for every instruction on a set of instructions.
pub struct Instruction {
    pub index: usize,
    pub kind: InstructionKind,
    pub times: usize
}

impl Instruction {
    pub fn new(index: usize, instruction_char: char) -> Self {
        let kind = match instruction_char {
            InstConsts::INC_PTR => InstructionKind::IncPtr,
            InstConsts::DEC_PTR => InstructionKind::DecPtr,
            InstConsts::INC_BYTE => InstructionKind::IncByte,
            InstConsts::DEC_BYTE => InstructionKind::DecByte,
            InstConsts::READ_BYTE => InstructionKind::ReadByte,
            InstConsts::WRITE_BYTE => InstructionKind::WriteByte,
            InstConsts::LOOP_START => InstructionKind::LoopStart {end_index: 0},
            InstConsts::LOOP_END => InstructionKind::LoopEnd {start_index: 0},
            InstConsts::REG_UP => InstructionKind::RegUp,
            InstConsts::REG_DOWN => InstructionKind::RegDown,
            InstConsts::ENV_OPEN => InstructionKind::EnvOpen,
            InstConsts::ENV_CLOSE => InstructionKind::EnvClose,
            InstConsts::COPY_FN => InstructionKind::CopyFn,
            InstConsts::IF_STATEM => InstructionKind::IfStatem,
            _ => panic!("Unrecognized command: {}", instruction_char)
        };
        Instruction {
            index,
            kind,
            times: 1
        }
    }

    pub fn add_repeat(&mut self) {
        self.times += 1;
    }
}

mod test {
}