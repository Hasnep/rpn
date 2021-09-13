use crate::cli::get_first_cli_arg;

pub struct ExpressionId {
    pub index: usize,
}

pub struct Expression {
    pub value: String,
    pub parent: Option<ExpressionId>,
}

impl Expression {
    pub fn get_value(&self) -> &str {
        return &self.value;
    }
}

// An arena to make sure all the nodes have the same lifetime
pub struct ExpressionStack {
    pub expressions: Vec<Expression>,
}

impl ExpressionStack {
    pub fn new() -> ExpressionStack {
        return ExpressionStack {
            expressions: Vec::new(),
        };
    }

    pub fn add_expression(
        mut self,
        parent_id: Option<ExpressionId>,
        expression_value: String,
    ) -> ExpressionId {
        let next_index = self.expressions.len();

        self.expressions.push(Expression {
            parent: parent_id,
            value: expression_value,
        });

        return ExpressionId { index: next_index };
    }

    // pub fn len(self) -> usize {
    //     return self.expressions.len();
    // }
}

mod cli;

fn main() {
    let stack_string = get_first_cli_arg();
    let stack_strings = stack_string.split_whitespace();

    let stack_expressions = &mut ExpressionStack::new();

    let root_id = stack_expressions.add_expression(None, "start".to_string());

    for s in stack_strings {
        stack_expressions.add_expression(Some(root_id), s.to_string());
    }

    for x in stack_expressions.expressions {
        println!("{}", x.value);
    }
}
