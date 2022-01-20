#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    IncrementPointer=0,
    DecrementPointer,
    PrintStatement,
    GetCStatement,
    LoopStart,
    LoopExit,
    IncrementValue,
    DecrementValue,
    UnknownOpcode
}

pub fn decode_to_opcodes(code: String) -> Vec<Opcode> {
    let mut code_operators = Vec::new();
    
    for char in code.chars() {
        let token =  match char {
            '>' => {Opcode::IncrementPointer},
            '<' => {Opcode::DecrementPointer},
            '+' => {Opcode::IncrementValue},
            '-' => {Opcode::DecrementValue},
            '[' => {Opcode::LoopStart},
            ']' => {Opcode::LoopExit},
            '.' => {Opcode::PrintStatement},
            ',' => {Opcode::GetCStatement},
            _ => {Opcode::UnknownOpcode},
        };
        code_operators.push(token);
    }
    code_operators
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    Start,
    PointerAddition(i32),
    PointerSubtraction(i32),
    PrintStatement(i32),
    GetCStatement(i32),
    LoopStart(i32),
    LoopExit,
    ValueAddition(i32),
    ValueSubtraction(i32),
    UnknownOpcode,
    EOF
}

#[derive(Clone)]
pub struct Node {
    pub instruction: Instruction,
    pub children: Vec<Option<Box<Node>>>
}

impl Node {
    pub fn new(instruction: Instruction) -> Node {
        Node {
            instruction: instruction,
            children: Vec::new()
        }
    }

    pub fn insert_child(&mut self, instruction: Instruction) {
        self.children.push(
            Some(Box::new(Node::new(instruction)))
        )
    }
}

pub fn build_tree(opcode_list: Vec<Opcode>){
    let mut root: Node = Node::new(Instruction::Start);
    let mut last_opcode = Opcode::UnknownOpcode;
    let mut current_node = &mut root;
    
    let mut loop_nodes: Vec<*mut Node> = Vec::new();
    let mut pointer_addr: i32 = 0;

    for opcode in opcode_list.iter() {
        match opcode {
            Opcode::IncrementValue => {
                if last_opcode == Opcode::IncrementValue 
                {
                    if let Instruction::ValueAddition(mut value) = current_node.instruction {
                        value += 1;
                    }
                } else {
                    current_node.insert_child(Instruction::ValueAddition(1));
                    current_node = current_node.children[0].as_mut().unwrap();
                }
            }

            Opcode::DecrementValue => {
                if last_opcode == Opcode::DecrementValue 
                {
                    if let Instruction::ValueSubtraction(mut value) = current_node.instruction {
                        value += 1;
                    }
                } else {
                    current_node.insert_child(Instruction::ValueSubtraction(1));
                    current_node = current_node.children[0].as_mut().unwrap();
                }
            }

            Opcode::IncrementPointer => {
                if last_opcode == Opcode::IncrementPointer 
                {
                    if let Instruction::PointerAddition(mut value) = current_node.instruction {
                        value += 1;
                        pointer_addr += 1;
                    }
                } else {
                    current_node.insert_child(Instruction::PointerAddition(1));
                    current_node = current_node.children[0].as_mut().unwrap();
                    pointer_addr += 1;
                }
            }

            Opcode::DecrementPointer => {
                if last_opcode == Opcode::DecrementPointer 
                {
                    if let Instruction::PointerSubtraction(mut value) = current_node.instruction {
                        value += 1;
                        pointer_addr -= 1;
                    }
                } else {
                    current_node.insert_child(Instruction::PointerSubtraction(1));
                    current_node = current_node.children[0].as_mut().unwrap();
                    pointer_addr -= 1;
                }
            }

            Opcode::LoopStart => { // Loops will have two branches: The loop branch and the "out of the loop" branch
                current_node.insert_child(Instruction::LoopStart(pointer_addr));
                current_node = current_node.children[0].as_mut().unwrap();
                loop_nodes.push(current_node);
            }

            Opcode::LoopExit => { 
                if loop_nodes.is_empty() {
                    panic!("No loop was open")
                }
                current_node = unsafe {&mut *loop_nodes.pop().unwrap()};
                current_node.insert_child(Instruction::LoopExit);
                current_node = current_node.children[1].as_mut().unwrap();
                
            }

            Opcode::PrintStatement => {
                current_node.insert_child(Instruction::PrintStatement(pointer_addr));
                current_node = current_node.children[0].as_mut().unwrap();
            }

            Opcode::GetCStatement => {
                current_node.insert_child(Instruction::GetCStatement(pointer_addr));
                current_node = current_node.children[0].as_mut().unwrap();
            }

            _ => {}
        }
        last_opcode = *opcode;
    }

    current_node.insert_child(Instruction::EOF);
}