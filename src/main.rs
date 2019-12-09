use parser;

fn main() {
    let result = parser::parse(r"ident = \x. $ x $ x y $ ident x");
}
