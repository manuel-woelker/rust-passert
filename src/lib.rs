#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]

#![feature(slice_patterns)]


extern crate syntax;
extern crate rustc;

use syntax::codemap::Span;
use syntax::parse::token::{self, str_to_ident};
use syntax::ast::{self, TokenTree, TtToken, Expr, Expr_};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::util::small_vector::SmallVector;
use syntax::codemap;
use syntax::ext::build::AstBuilder;  // trait for expr_usize
use syntax::print::pprust;  // trait for expr_usize
use rustc::plugin::Registry;
use syntax::ptr::P;

fn expand_passert(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
        -> Box<MacResult + 'static> {
/*
    static NUMERALS: &'static [(&'static str, u32)] = &[
        ("M", 1000), ("CM", 900), ("D", 500), ("CD", 400),
        ("C",  100), ("XC",  90), ("L",  50), ("XL",  40),
        ("X",   10), ("IX",   9), ("V",   5), ("IV",   4),
        ("I",    1)];

    let text = match args {
        [TtToken(_, token::Ident(s, _))] => token::get_ident(s).to_string(),
        _ => {
            cx.span_err(sp, "argument should be a single identifier");
            return DummyResult::any(sp);
        }
    };

    let mut text = &*text;
    let mut total = 0;
    while !text.is_empty() {
        match NUMERALS.iter().find(|&&(rn, _)| text.starts_with(rn)) {
            Some(&(rn, val)) => {
                total += val;
                text = &text[rn.len()..];
            }
            None => {
                cx.span_err(sp, "invalid Roman numeral");
                return DummyResult::any(sp);
            }
        }
    }

    MacEager::expr(cx.expr_u32(sp, total))
    */
    println!("///////////////////////////////////////////////////////////");
    println!("tt: {:?}", args);

    let mut parser = cx.new_parser_from_tts(args);

    let expr: P<Expr> = parser.parse_expr();
    cx.span_note(sp, "Foobar was here");
    if let Expr_::ExprBinary(ref op, ref a, ref b) = (*expr).node {
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
        println!("a:  {:?}", a.span);
        println!("OP: {:?}", op);
        println!("b:  {:?}", b.span);
    }
    if let Expr_::ExprMac(ref mac) = (*expr).node {
        println!("QQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQ");
        println!("mac: {:?}", mac);
    }

    //path, tts, EMPTY_CTXT
    let panic_path = cx.path(sp, vec!(str_to_ident("panic")));
    let literal = token::Token::Literal(token::Str_(token::intern("testing only {:?}")), Option::None);
    let tt_string = TtToken(sp, literal);
//    let panic_args = TtSequence(sp, Rc::new(SequenceRepetition {tts: vec!(token_tree)}));
    let result_ident = str_to_ident("result");
    let tt_arg = TtToken(sp, token::Ident(result_ident, token::Plain));
    let tt_comma = TtToken(sp, token::Comma);
    let my_mac = codemap::respan(sp, ast::MacInvocTT(panic_path, vec!(tt_string, tt_comma, tt_arg), ast::EMPTY_CTXT));
    println!("my_mac: {:?}", my_mac);
    let my_panic = ast::ExprMac(my_mac);
    let my_panic_expr = cx.expr(sp, my_panic);
    println!("Expression: {:?}", expr);
    println!("Span: {:?}", sp);
    let s = pprust::tts_to_string(args);
    println!("Exp string: {}", s);
//    MacEager::expr(cx.expr_u32(sp, 2014))
    let mut stmts = Vec::new();
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
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("passert", expand_passert);
}



#[test]
fn it_works() {
//     assert_eq!(rn!(MMXV), 2015);
}
