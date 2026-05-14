#[derive(Debug, Clone, Copy)]
enum Instruction {
    Push(i64),
    Add,
    Mul,
}

fn run(program: &[Instruction]) -> Result<i64, String> {
    let mut stack = Vec::new();

    for instruction in program {
        match *instruction {
            Instruction::Push(value) => stack.push(value),
            Instruction::Add => {
                let right = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                let left = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                stack.push(left + right);
            }
            Instruction::Mul => {
                let right = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                let left = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                stack.push(left * right);
            }
        }
    }

    match stack.as_slice() {
        [value] => Ok(*value),
        _ => Err(format!("expected one result, got stack: {stack:?}")),
    }
}

fn main() {
    let program = [
        Instruction::Push(1),
        Instruction::Push(2),
        Instruction::Push(3),
        Instruction::Mul,
        Instruction::Add,
    ];

    println!("program: {:?}", program);
    println!("result: {:?}", run(&program));
}
