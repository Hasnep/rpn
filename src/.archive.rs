// pub struct ExpressionId {
//     pub index: usize,
// }

// pub struct Expression {
//     pub value: String,
//     pub parent: Option<ExpressionId>,
// }

// // An arena to make sure all the nodes have the same lifetime
// pub struct ExpressionStack {
//     pub expressions: Vec<Expression>,
// }

// impl ExpressionStack {
//     pub fn new() -> ExpressionStack {
//         return ExpressionStack {
//             expressions: Vec::new(),
//         };
//     }

//     pub fn add_expression(
//         &mut self,
//         parent_id: Option<ExpressionId>,
//         expression_value: String,
//     ) -> ExpressionId {
//         let next_index = self.expressions.len();

//         self.expressions.push(Expression {
//             parent: parent_id,
//             value: expression_value,
//         });

//         return ExpressionId { index: next_index };
//     }
// }

