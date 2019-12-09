pub mod ast;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(LambdaScript);

pub fn parse(input: &str) -> ast::Program {
    LambdaScript::ProgramParser::new().parse(input).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
