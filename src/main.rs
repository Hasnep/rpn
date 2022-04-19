use crate::cli::get_cli_args;

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
        return match self.children.len() {
            0 => self.value,
            1 => [
                &self.value,
                "(",
                self.children
                    .into_iter()
                    .map(|e| e.to_string())
                    .next()
                    .unwrap()
                    .as_str(),
                ")",
            ]
            .join(""),
            2 => self
                .children
                .into_iter()
                .map(|e| e.to_string())
                .rev()
                .collect::<Vec<_>>()
                .join(&[" ", &self.value, " "].join("")),
            _ => "Something went wrong!".to_string(),
        };
    }
}

fn push_expression_to_stack(stack: &mut Vec<Expression>, i: String) {
    return match i.as_str() {
        "+" | "*" | "-" | "/" | "^" => {
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            let new_expression = Expression {
                value: i,
                children: vec![a, b],
            };
            stack.push(new_expression);
        }
        "cos" | "exp" | "ln" | "log" | "sin" | "sqrt" | "tan" => {
            let x = stack.pop().unwrap();
            let new_expression = Expression {
                value: i,
                children: vec![x],
            };
            stack.push(new_expression);
        }
        _ => stack.push(Expression::new(i.to_string())),
    };
}

fn main() {
    let input_stack = get_cli_args();
    let mut stack: Vec<Expression> = Vec::new();
    for i in input_stack {
        push_expression_to_stack(&mut stack, i);
    }

    // Print stack
    println!("Stack: ");
    for e in stack {
        println!("{}", e.to_string());
    }
}
