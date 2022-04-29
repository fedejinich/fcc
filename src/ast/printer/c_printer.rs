// impl ASTItem for ConstantExpression {
//     fn generate_assembly(&self) -> String {
//         self.constant.to_string()
//     }

//     fn pretty_print(&self) -> String {
//         format!("Int<{}>", self.constant)
//     }

//     fn pretty_print_2(&self) -> String {
//         format!("Const({})", self.constant)
//     }
// }

// impl ASTItem for FunctionDefinition {
//     fn generate_assembly(&self) -> String {
//         panic!("should be implemented")
//     }

//     fn pretty_print(&self) -> String {
//         format!(
//             "FUN {}:\n    {}",
//             self.name,
//             self.body
//                 .iter()
//                 .map(|s| s.pretty_print())
//                 .fold(String::from(""), |acc, s| format!("{}{}", acc, s))
//         )
//     }

//     fn pretty_print_2(&self) -> String {
//         format!(
//             "Function(\n    name=\"{}\",\n    body={}",
//             self.name,
//             self.body
//                 .iter()
//                 .map(|s| s.pretty_print_2())
//                 .fold(String::from(""), |acc, s| format!("{}{}", acc, s))
//         )
//     }
// }

// impl ASTItem for Program {
//     fn generate_assembly(&self) -> String {
//         self.function_declaration.generate_assembly()
//     }

//     fn pretty_print(&self) -> String {
//         format!("PROGRAM:\n  {}", self.function_declaration.pretty_print())
//     }

//     fn pretty_print_2(&self) -> String {
//         format!(
//             "Program(\n  {}\n)",
//             self.function_declaration.pretty_print_2()
//         )
//     }
// }

// todo(fedejinich) should be ReturnStatement
// impl Statement {
//     pub fn new(expression: Expression) -> Statement {
//         Statement { expression }
//     }
// }

// impl ASTItem for ReturnStatement {
//     fn generate_assembly(&self) -> String {
//         format!(
//             "movl    ${}, %eax\nret",
//             self.expression.generate_assembly()
//         )
//         .to_string()
//     }

//     fn pretty_print(&self) -> String {
//         format!("RETURN {}", self.expression.pretty_print())
//     }

//     fn pretty_print_2(&self) -> String {
//         format!("Return(\n      {}\n    )", self.expression.pretty_print_2())
//     }
// }
