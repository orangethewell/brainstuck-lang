use crate::ast::error_handler::Error;
use crate::ast::instruction::{Instruction, INSTS, InstConsts, InstructionKind};

pub fn parse_source(src: &str) -> Result<Vec<Instruction>, Error> {
    let mut loop_depth: usize = 0;
    // TODO: Make env_flag detection
    let mut env_flag: bool = false;
    let mut insts: Vec<Instruction> = Vec::new();

    // 1st pass, convert chars to instructions
    for c in src.chars().filter(|c| INSTS.contains(c)) {
        if c == InstConsts::LOOP_END {
            if loop_depth == 0 {
                return Err(Error::NonClosedBrackets);
            }
            loop_depth -= 1;
        } else if c == InstConsts::LOOP_START {
            loop_depth += 1;
        }

        let curr_inst = Instruction::new(insts.len(), c);
        if let Some(prev_inst) = insts.last_mut() {
            if prev_inst.kind == curr_inst.kind {
                prev_inst.add_repeat();
            } else {
                insts.push(curr_inst);
            }
        } else {
            insts.push(curr_inst);
        }
    }

    if loop_depth > 0 {
        return Err(Error::NonClosedBrackets);
    }

    // 2nd pass, link loops together by setting their jump indexes
    for i in 0..insts.len() {
        let mut update_jump_index: Option<usize> = None;
        let Instruction {kind, times, ..} = &insts[i];
        match kind {
            InstructionKind::LoopStart { .. } => {
                let mut loop_starts = *times;

                for j in i+1..insts.len(){
                    let Instruction {kind, times, ..} = &insts[j];
                    match kind {
                        InstructionKind::LoopEnd { .. } => {
                            let loop_ends = *times;
                            loop_starts = loop_starts.saturating_sub(loop_ends);
                            if loop_starts == 0 {
                                update_jump_index = Some(j - 1);
                                break
                            }
                        },
                        InstructionKind::LoopStart { .. } => {
                            let nested_loop_starts = *times;
                            loop_starts += nested_loop_starts;
                        },

                        _ => (),
                    }
                }
            },

            InstructionKind::LoopEnd { .. } => {
                let mut loop_ends = 1_usize;

                for j in (0..i).rev() {
                    let Instruction {kind, times, .. } = &insts[j];
                    match kind {
                        InstructionKind::LoopStart { .. } => {
                            let loop_starts = *times;
                            loop_ends = loop_ends.saturating_sub(loop_starts);
                            if loop_ends == 0 {
                                if i == j + 1 {
                                    return Err(Error::InfiniteLoop);
                                }
                                update_jump_index = Some(j + 1);
                                break;
                            }
                        },

                        InstructionKind::LoopEnd { .. } => {
                            let nested_loop_ends = *times;
                            loop_ends += nested_loop_ends;
                        },
                        _ => (),
                    }
                }
            },
            _ => (),
        }
        if let Some(jump_index) = update_jump_index {
            insts[i].kind.add_jump_index(jump_index);
        }
    }
    return Ok(insts)
}