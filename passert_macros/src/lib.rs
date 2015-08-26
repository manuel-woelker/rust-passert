#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]

#![feature(slice_patterns)]


extern crate syntax;
extern crate rustc;

use syntax::codemap::{Span, Pos};
use syntax::parse::token::{self, str_to_ident};
use syntax::ast::{self, TokenTree, TtToken, Expr, Expr_, Stmt};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::util::small_vector::SmallVector;
use syntax::codemap;
use syntax::ext::build::AstBuilder;  // trait for expr_usize
use syntax::print::pprust;  // trait for expr_usize
use rustc::plugin::Registry;
use syntax::ptr::P;

fn expand_passert(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
        -> Box<MacResult + 'static> {
    println!("///////////////////////////////////////////////////////////");
    let s = pprust::tts_to_string(args);
    println!("Exp string:  {}", s);
//    println!("tt: {:?}", args);

    let mut parser = cx.new_parser_from_tts(args);
    let expr: P<Expr> = parser.parse_expr();
    let span_string = cx.codemap().span_to_snippet(expr.span).unwrap();
    println!("Span string: {}", span_string);

    let mut assertion_helper = AssertionHelper::new();

    let root_expr = assertion_helper.collect_expression(&expr, cx);

    let mut stmts = Vec::new();
    stmts.extend(assertion_helper.statements);

    let condition = cx.expr_unary(sp, ast::UnOp::UnNot, root_expr);

    // Panic path
    let literal = token::Token::Literal(token::Str_(token::intern(&format!("Assertion failed: {}", span_string))), Option::None);
    let tt_string = TtToken(sp, literal);
    let panic_path = cx.path(sp, vec!(str_to_ident("panic")));
    let my_mac = codemap::respan(sp, ast::MacInvocTT(panic_path, vec!(tt_string), ast::EMPTY_CTXT));
    let my_panic = ast::ExprMac(my_mac);
    let my_panic_expr = cx.expr(sp, my_panic);

    let then_expr = my_panic_expr;
    // Check statement
    stmts.push(cx.stmt_expr(cx.expr_if(sp, condition, then_expr, Option::None)));

    // assertion block
    let block = cx.block(sp, stmts, Option::None);
    let expr_block = cx.expr_block(block);
    MacEager::expr(expr_block)

/*
    let first_column = cx.codemap().lookup_char_pos(expr.span.lo).col.to_usize();
    println!("START POS: {}", first_column);
    cx.span_note(sp, "Foobar was here");
    let mut stmts = Vec::new();
    let temp1_ident = str_to_ident("temp1");
    let temp2_ident = str_to_ident("temp2");
    let temp1_str_ident = str_to_ident("temp1str");
    if let Expr_::ExprBinary(ref op, ref a, ref b) = (*expr).node {
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
        println!("a:  {:?}", a.span);
        println!("OP: {:?}", op);
        println!("b:  {:?}", b.span);
        println!("B POS: {}", cx.codemap().lookup_char_pos(b.span.lo).col.to_usize());
        stmts.push(cx.stmt_let(sp, false, temp1_ident, a.clone()));
        stmts.push(cx.stmt_let(sp, false, temp2_ident, b.clone()));

        // Format left side of expression
        let format_path = cx.path(sp, vec!(str_to_ident("format")));
        let format_string = TtToken(sp, token::Token::Literal(token::Str_(token::intern("x{:?}x")), Option::None));

        let tt_arg = TtToken(sp, token::Ident(temp1_ident, token::Plain));
        let tt_comma = TtToken(sp, token::Comma);
        let my_mac = codemap::respan(sp, ast::MacInvocTT(format_path, vec!(format_string, tt_comma, tt_arg), ast::EMPTY_CTXT));
        println!("my_mac: {:?}", my_mac);
        let my_format = ast::ExprMac(my_mac);
        let my_format_expr = cx.expr(sp, my_format);

        stmts.push(cx.stmt_let(sp, false, temp1_str_ident, my_format_expr));
    }
    if let Expr_::ExprMac(ref mac) = (*expr).node {
        println!("QQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQ");
        println!("mac: {:?}", mac);
    }
*/

/*
    //path, tts, EMPTY_CTXT
    let panic_path = cx.path(sp, vec!(str_to_ident("panic")));
    let literal = token::Token::Literal(token::Str_(token::intern("testing only {:?}")), Option::None);
    let tt_string = TtToken(sp, literal);
//    let panic_args = TtSequence(sp, Rc::new(SequenceRepetition {tts: vec!(token_tree)}));
    let result_ident = str_to_ident("result");
//    let tt_arg = TtToken(sp, token::Ident(result_ident, token::Plain));
    let tt_arg = TtToken(sp, token::Ident(temp1_str_ident, token::Plain));
    let tt_comma = TtToken(sp, token::Comma);
    let my_mac = codemap::respan(sp, ast::MacInvocTT(panic_path, vec!(tt_string, tt_comma, tt_arg), ast::EMPTY_CTXT));
    println!("my_mac: {:?}", my_mac);
    let my_panic = ast::ExprMac(my_mac);
    let my_panic_expr = cx.expr(sp, my_panic);
    println!("Expression: {:?}", expr);
    println!("Span: {:?}", sp);
//    MacEager::expr(cx.expr_u32(sp, 2014))
    stmts.push(cx.stmt_let(sp, false, result_ident, expr));
    stmts.push(cx.stmt_expr(my_panic_expr));

    let condition = cx.expr_unary(sp, ast::UnOp::UnNot, cx.expr_ident(sp, result_ident));
    let then_expr = cx.expr_u32(sp, 1234);
//    let panic_mac = codemap::respan(sp, )
    //let then_expr = ast::ExprMac()
//    stmts.push(cx.stmt_expr(cx.expr_if(sp, condition, then_expr, Option::None)));
    let block = cx.block(sp, stmts, Option::None);
    let expr_block = cx.expr_block(block);
    MacEager::expr(expr_block)
//    MacEager::expr(expr)
//    MacEager::expr(my_panic_expr)
//    MacEager::stmts(SmallVector::one(cx.stmt_let(sp, false, str_to_ident("foo"), expr)))

//    return DummyResult::any(sp);
*/
}

struct Expression {
    column_offset: usize,
    var_name: String
}

struct AssertionHelper {
    intermediate_counter: usize,
    statements: Vec<P<Stmt>>
}

impl AssertionHelper {
    fn new() -> AssertionHelper {
        AssertionHelper {intermediate_counter: 0, statements: Vec::new()}
    }

    fn collect_expression(&mut self, expr: &P<Expr>, cx: &mut ExtCtxt) -> P<Expr> {
        println!("Collecting expression: {:?}", expr);
        match (*expr).node {
            Expr_::ExprBinary(ref op, ref a, ref b) => {
                println!("BINARY");
                let new_a = self.collect_expression(a, cx);
                let new_b = self.collect_expression(b, cx);
                let new_expr = cx.expr_binary(expr.span, op.node, new_a, new_b);
                self.create_let_statement(&new_expr, cx)
            }
            _ => {
                self.create_let_statement(expr, cx)
            }
        }
        /*
        if let Expr_::ExprBinary(ref op, ref a, ref b) = (*expr).node {
            stmts.push(cx.stmt_let(sp, false, temp1_ident, a.clone()));
            stmts.push(cx.stmt_let(sp, false, temp2_ident, b.clone()));

            // Format left side of expression
            let format_path = cx.path(sp, vec!(str_to_ident("format")));
            let format_string = TtToken(sp, token::Token::Literal(token::Str_(token::intern("x{:?}x")), Option::None));

            let tt_arg = TtToken(sp, token::Ident(temp1_ident, token::Plain));
            let tt_comma = TtToken(sp, token::Comma);
            let my_mac = codemap::respan(sp, ast::MacInvocTT(format_path, vec!(format_string, tt_comma, tt_arg), ast::EMPTY_CTXT));
            println!("my_mac: {:?}", my_mac);
            let my_format = ast::ExprMac(my_mac);
            let my_format_expr = cx.expr(sp, my_format);

            stmts.push(cx.stmt_let(sp, false, temp1_str_ident, my_format_expr));
        }
*/
    }

    fn create_let_statement(&mut self, expr: &P<Expr>, cx: &mut ExtCtxt) -> P<Expr> {
        let var_name = format!("temp_{}", self.intermediate_counter);
        self.intermediate_counter += 1;
        let ident = str_to_ident(&var_name);
        let let_stmt = cx.stmt_let(expr.span, false, ident, expr.clone());
        println!("LET {:?} @{} {}", let_stmt, cx.codemap().lookup_char_pos(expr.span.lo).col.to_usize()+1, stringify!(a+b  +  c));
        self.statements.push(let_stmt);
        cx.expr_ident(expr.span, ident)
    }

}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("passert", expand_passert);
}



#[test]
fn it_works() {
//     assert_eq!(rn!(MMXV), 2015);
}
