use parser::ast;
use parser::ast::Expr;
use parser::ast::Identifier;
use parser::ast::Stmt;

pub struct Compiler<'pro> {
    pub program: &'pro ast::Program,
    pub result: &'pro mut String,
}

impl<'pro> Compiler<'pro> {
    pub fn compile(&mut self) {
        for stmt in self.program {
            match stmt {
                Stmt::ExprStmt(expr) => self.compile_expr_stmt(expr),
                Stmt::DefStmt(def_stmt) => self.compile_def_stmt(def_stmt),
            };
            self.build(";\n");
        }

        println!("{}", &self.result);
    }

    fn compile_def_stmt(&mut self, def: &ast::Def) {
        self.build("const ");
        let name = match &def.name {
            Identifier::Identifier(name) => name,
            Identifier::ExternIdentifier(name) => name,
        };
        self.build(&name);
        self.build(" = ");
        self.compile_expr(&def.expr);
    }

    fn compile_expr_stmt(&mut self, expr: &ast::Expr) {
        self.compile_expr(expr);
    }

    fn compile_expr(&mut self, expr: &ast::Expr) {
        match expr {
            Expr::Identifier(ident) => self.compile_identifier(ident),
            Expr::Abstruction(abstruction) => self.compile_abstruction(abstruction),
            Expr::Application(application) => self.compile_application(application),
            Expr::Literal(literal) => self.compile_literal(literal),
        }
    }

    fn compile_literal(&mut self, literal: &ast::Literal) {
        self.build(&literal);
    }

    fn compile_application(&mut self, application: &ast::Application) {
        self.compile_expr(&application.left);

        self.build("(");
        self.compile_expr(&application.right);
        self.build(")");
    }

    fn compile_identifier(&mut self, ident: &ast::Identifier) {
        let id = match ident {
            Identifier::Identifier(id) => id,
            Identifier::ExternIdentifier(id) => id,
        };

        self.build(&format!("{}", id));
    }

    fn compile_abstruction(&mut self, abst: &ast::Abstruction) {
        for (index, arg) in abst.args.iter().enumerate() {
            let name = match arg {
                Identifier::Identifier(name) => name,
                Identifier::ExternIdentifier(name) => name,
            };
            self.build(&format!("({})", name));
            self.build(" => ");
            if index == abst.args.len() - 1 {
                self.compile_expr(&abst.body);
            }
        }
    }

    // fn build_function(&mut self, args: &mut ast::Args, body: ast::Expr) {
    //     let last = args.len() <= 1;

    //     match args.first() {
    //         Some(arg) => {
    //             self.build(arg);
    //         }
    //         None => (self.build("()")),
    //     }
    //     self.build(" => { return ");

    //     if last {
    //         self.compile_expr(&body);
    //     } else {
    //         args.remove(0);
    //         self.build_function(args, body)
    //     }
    // }

    fn build(&mut self, string: &str) {
        self.result.push_str(&format!("{}", string));
    }
}
