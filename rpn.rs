use std::io::{self, Read};

fn main() {
    let mut stack: Vec<i32> = Vec::new();
     
    loop {
        // Puts input into byte vector until space or newline is reached
        let stdin = io::stdin();
        let mut buffer = Vec::new();
        let mut exit = false;

        let mut iter = stdin.bytes();
        loop {
            match iter.next() {
                Some(byte) => {
                    let byte = byte.unwrap();
                    if byte == b' ' || byte == b'\n' {
                        break;
                    }
                    else {
                        buffer.push(byte);
                    }
                },
                None => { 
                    exit = true;
                    break;
                },
            }
        }

        // Handles cases with multiple spaces or newlines
        if buffer.len() == 0 && exit == false { continue; }

        // Converts byte vector to string
        let string = String::from_utf8(buffer).unwrap();

        // Tests if string is an operator
        if string.eq("+") || string.eq("-") || string.eq("*") || string.eq("/") {
            if stack.len() < 2 {
                break;
            }
            let operand1 = stack.pop().unwrap();
            let operand2 = stack.pop().unwrap();
            if string.eq("+") {
                stack.push(operand2 + operand1);
            }
            else if string.eq("-") {
                stack.push(operand2 - operand1);
            }
            else if string.eq("*") {
                stack.push(operand2 * operand1);
            }
            else if string.eq("/") {
                stack.push(operand2 / operand1);
            }
        }

        // Pushes string to stack if valid; exits otherwise
        else {
            match string.parse() {
                Ok(n) => stack.push(n),
                Err(_e) => { println!("ERROR: Invalid input"); break; },
            }
        }
        
        // Exits program if EOF is reached
        if exit == true {
            break;
        }
    }
    print_stack(&mut stack);
}

fn print_stack(stack: &mut Vec<i32>) {
    print!("[ ");
    for (i, item) in stack.iter().enumerate() {
        if i + 1 == stack.len() {
            print!("{} ", item);
        }
        else {
            print!("{}, ", item);
        }
    }
    println!("]");
}
