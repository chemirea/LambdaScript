mod compiler;
use parser;

fn main() {
    let result = &parser::parse(
        r"
        true = \t f. t
        false = \t f. f
        not = \x. $$ x false true
        and = \x y. $$ x y false
        xor = \x y. $ x $$ not y y

        if = \c x y . $$ c x y

        one = 1
        zero = 0

        tmp_false = $ not true

        result = $$$ if tmp_false one zero

        $ !console.log result
        ",
    );
    dbg!(result);

    let mut compiler = compiler::Compiler {
        program: result,
        result: &mut String::from(""),
    };
    compiler.compile();
}
