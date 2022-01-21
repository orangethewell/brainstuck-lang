use colored::Colorize;

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
        if token != Opcode::UnknownOpcode {
            code_operators.push(token)
        }
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
    pub index: i32,
    pub children: Vec<Option<Box<Node>>>,
}

impl Node {
    pub fn new(instruction: Instruction, index: i32) -> Node {
        Node {
            instruction: instruction,
            index: index,
            children: Vec::new()
        }
    }

    pub fn insert_child(&mut self, instruction: Instruction, index: i32) {
        self.children.push(
            Some(Box::new(Node::new(instruction, index)))
        )
    }

    pub fn find_child_from_index(&self, index: i32) -> Option<&Node> {
        let mut current_node = self;
        let mut childrened_nodes: Vec<*const Node> = Vec::new();

        loop {
            if !current_node.children.is_empty() {
                if current_node.children.len() > 1 {
                    childrened_nodes.push(*&current_node)
                }
                current_node = &*current_node.children[0].as_ref().unwrap();
            } else {
                if !childrened_nodes.is_empty(){
                    current_node = unsafe{&*childrened_nodes.pop().unwrap()}
                        .children[1].as_ref().unwrap()
                } else {
                    return None;
                }
            }
            if current_node.index == index {
                break
            }
        }
        Some(current_node)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ExceptionFlag {
    NonClosedLoop,
    NonOpenLoop
}

pub struct ExceptTrace {
    index: i32,
    exc_type: ExceptionFlag,
    previous_line: String,
    main_instruction: String,
    next_line: String,
    reason: String
}

pub fn getc_for_opcode(opcode: Opcode) -> char {
    match opcode {
        Opcode::IncrementValue => '+',
        Opcode::DecrementValue => '-',
        Opcode::IncrementPointer => '>',
        Opcode::DecrementPointer => '<',
        Opcode::LoopStart => '[',
        Opcode::LoopExit => ']',
        Opcode::PrintStatement => '.',
        _ => '\0'
    }
}

pub fn get_reason(except: ExceptionFlag) -> String {
    String::from(
        match except {
            ExceptionFlag::NonClosedLoop => "One or more loops was caught without closed pair. Try adding a ']' somewhere.",
            ExceptionFlag::NonOpenLoop => "A ']' was found in code, remove it or add a opening bracket before it. ('[')"
    })
}

pub fn raise_exception(exception: ExceptTrace){
    println!("{}", format!("~~~~~~ Error on building! ~~~~~~\n").bold().red());
    println!("error on {}: \n|\t{}{}{}", 
        format!("instruction {}", exception.index).bold(), 
        format!("{}", exception.previous_line).bright_blue(),
        format!("{}", exception.main_instruction).bright_red(),
        format!("{}", exception.next_line).bright_blue()
    );
    println!("|\t{}{}{}\n", 
        format!("{}", "~".repeat(exception.previous_line.len())).yellow(),
        format!("{}", "^").red(),
        format!("{}", "~".repeat(exception.next_line.len())).yellow()
    );
    println!("{}", format!("{}", exception.reason).bold());
    println!("{}\n", format!("Exception: {:?}", exception.exc_type).bright_blue());
}

pub fn build_exception(instruction_set: &Vec<Opcode>, except: ExceptionFlag, index: usize) -> ExceptTrace{
    let error_instruction = instruction_set[index - 1];
    let mut next_nodes: Vec<Opcode> = Vec::new();
    for i in index..index+5 {
        next_nodes.push(instruction_set[i])
    }

    let mut previous_nodes: Vec<Opcode> = Vec::new();
    for i in index-5..index-1 {
        previous_nodes.push(instruction_set[i])
    }

    let mut next_line = String::from("");
    for n in next_nodes.iter(){
        next_line.push(getc_for_opcode(*n));
    }
    let mut previous_line = String::from("");
    for n in previous_nodes.iter(){
        previous_line.push(getc_for_opcode(*n));
    }

    ExceptTrace {
        index: index as i32,
        exc_type: except,
        previous_line: previous_line,
        main_instruction: getc_for_opcode(error_instruction).to_string(),
        next_line: next_line,
        reason: get_reason(except)
    }
}


pub fn build_tree(opcode_list: Vec<Opcode>) -> Result<Node, i32>{
    let mut root: Node = Node::new(Instruction::Start, 0);
    let mut loop_nodes: Vec<*mut Node> = Vec::new();
    let mut exceptions: Vec<ExceptTrace> = Vec::new();
    
    let mut loop_err_flag: bool = false;

    let mut index: i32 = 1;
    {
        let mut last_opcode = Opcode::UnknownOpcode;
        let mut current_node = &mut root;
        
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
                        current_node.insert_child(Instruction::ValueAddition(1), index);
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
                        current_node.insert_child(Instruction::ValueSubtraction(1), index);
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
                        current_node.insert_child(Instruction::PointerAddition(1), index);
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
                        current_node.insert_child(Instruction::PointerSubtraction(1), index);
                        current_node = current_node.children[0].as_mut().unwrap();
                        pointer_addr -= 1;
                    }
                }

                Opcode::LoopStart => { // Loops will have two branches: The loop branch and the "out of the loop" branch
                    current_node.insert_child(Instruction::LoopStart(pointer_addr), index);
                    current_node = current_node.children[0].as_mut().unwrap();
                    loop_nodes.push(current_node);
                }

                Opcode::LoopExit => { 
                    if loop_nodes.is_empty() {
                        current_node.insert_child(Instruction::LoopExit, index);
                        exceptions.push(build_exception(&opcode_list, ExceptionFlag::NonOpenLoop, index as usize));
                    } else {
                        current_node = unsafe {&mut *loop_nodes.pop().unwrap()};
                        current_node.insert_child(Instruction::LoopExit, index);
                        current_node = current_node.children[1].as_mut().unwrap();
                    }
                }

                Opcode::PrintStatement => {
                    current_node.insert_child(Instruction::PrintStatement(pointer_addr), index);
                    current_node = current_node.children[0].as_mut().unwrap();
                }

                Opcode::GetCStatement => {
                    current_node.insert_child(Instruction::GetCStatement(pointer_addr), index);
                    current_node = current_node.children[0].as_mut().unwrap();
                }

                _ => {}
            }
            last_opcode = *opcode;
            index += 1;
        }
        current_node.insert_child(Instruction::EOF, index + 1);
    }
    if !loop_nodes.is_empty(){
        exceptions.push(build_exception(&opcode_list, ExceptionFlag::NonClosedLoop, unsafe {&mut *loop_nodes[0]}.index as usize));
    }

    if !exceptions.is_empty() {
        for except in exceptions {
            raise_exception(except); 
        }
        return Err(-1);
    }

    return Ok(root);
}