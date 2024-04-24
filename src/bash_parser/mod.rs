/// error used by other modules
pub mod error;
/// creates tokens from input
pub mod lexer;
/// utilizes submodules to create a parser tree
pub mod parser;
/// yields requested elements from input for other modules
pub mod stream;
