fn main() {
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let words: Vec<_> = line.split(" ").collect();

            println!("Line: {words:?}")
        }
    }
}

fn add(stack: &mut Vec<i32>) {
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();

    stack.push(lhs + rhs);
}
