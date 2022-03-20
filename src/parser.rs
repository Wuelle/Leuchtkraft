use pest::Parser;

use crate::error::Error;
use crate::ast::AstNode;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LEParser;

pub fn parse_str(unparsed_file: &str) -> Result<AstNode, Error> {
    log::info!(target: "Parser", "Calling pest");
    let tree = LEParser::parse(Rule::Program, &unparsed_file)?.next().unwrap();
    log::info!(target: "Parser", "Parsing AST");
    AstNode::from_tree(tree)
}
