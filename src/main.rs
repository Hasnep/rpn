use crate::cli::get_first_cli_arg;

mod cli;

pub struct Expression {
    pub value: String,
    pub children: Vec<Expression>,
}

impl Expression {
    pub fn new(value: String) -> Expression {
        return Expression {
            value: value,
            children: Vec::new(),
        };
    }

    fn to_string(self) -> String {
        if self.children.len() == 0 {
            return self.value;
        } else if self.children.len() == 2 {
            return self
                .children
                .into_iter()
                .map(|e| e.to_string())
                .rev()
                .collect::<Vec<_>>()
                .join(&[" ", &self.value, " "].join(""));
        } else {
            return "Something went wrong!".to_string();
        }
    }
}

fn main() {
    let input_stack = get_first_cli_arg();
    let input_stack = input_stack.split_whitespace();
    let mut stack = Vec::new();
    for i in input_stack {
        match i {
            "+" | "*" | "-" | "/" | "^" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let mut children = Vec::new();
                children.push(a);
                children.push(b);
                let mut new_expression = Expression::new(i.to_string());
                new_expression.children = children;
                stack.push(new_expression);
            }
            _ => stack.push(Expression::new(i.to_string())),
        }
    }

    // Print stack
    println!("Stack: ");
    for e in stack {
        println!("{}", e.to_string());
    }
}
