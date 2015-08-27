#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]

#![feature(slice_patterns)]


extern crate syntax;
extern crate rustc;

use syntax::codemap::{Span, Pos};
use syntax::parse::token::{self, str_to_ident};
use syntax::ast::{self, TokenTree, TtToken, Expr, Expr_, Stmt};
use syntax::ext::base::{ExtCtxt, MacResult, MacEager};
use syntax::codemap;
use syntax::ext::build::AstBuilder;
use rustc::plugin::Registry;
use syntax::ptr::P;

fn expand_passert(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
        -> Box<MacResult + 'static> {

    let mut parser = cx.new_parser_from_tts(args);
    let expr: P<Expr> = parser.parse_expr();
    let span_string = cx.codemap().span_to_snippet(expr.span).unwrap();

    let mut expression_collector = ExpressionCollector::new();

    let root_expr = expression_collector.collect_expression(&expr, cx);

    let mut stmts = Vec::new();
    stmts.extend(expression_collector.statements);

    let condition = cx.expr_unary(sp, ast::UnOp::UnNot, root_expr);

    // Panic path
    let literal = token::Token::Literal(token::Str_(token::intern(&format!("Assertion failed: {}", span_string))), Option::None);
    let tt_string = TtToken(sp, literal);
    let panic_expr = create_macro_call(cx, sp, "panic", vec!(tt_string));

    let mut then_stmts = Vec::new();
    // Create helper
    let helper_ident = str_to_ident("helper");
    let args = vec!(
        cx.expr_usize(sp, cx.codemap().lookup_char_pos(expr.span.lo).col.to_usize()),
        cx.expr_str(sp, token::intern_and_get_ident(&span_string)));
    let new_call = cx.expr_call_global(sp, vec!(str_to_ident("passert"),str_to_ident("PassertHelper"),str_to_ident("new")), args);
    let let_stmt = cx.stmt_let(sp, true, helper_ident, new_call);

    then_stmts.push(let_stmt);

    for expression in expression_collector.expressions {
        let literal = token::Token::Literal(token::Str_(token::intern("{:?}")), Option::None);
        let tt_string = TtToken(sp, literal);
        let tt_comma = TtToken(sp, token::Comma);
        let tt_arg = TtToken(sp, token::Ident(str_to_ident(&expression.var_name), token::Plain));
        let format_expr = create_macro_call(cx, sp, "format", vec!(tt_string, tt_comma, tt_arg));
        let args = vec!(cx.expr_usize(sp, expression.column_offset), format_expr);
        let call_expr = cx.expr_method_call(sp, cx.expr_ident(sp, helper_ident), str_to_ident("add_expression"), args);
        then_stmts.push(cx.stmt_expr(call_expr));
    }

    let print_result_expr = cx.expr_method_call(sp, cx.expr_ident(sp, helper_ident), str_to_ident("print_result"), Vec::new());
    then_stmts.push(cx.stmt_expr(print_result_expr));

    then_stmts.push(cx.stmt_expr(panic_expr));

    let then_expr = cx.expr_block(cx.block(sp, then_stmts, Option::None));
    // Check statement
    stmts.push(cx.stmt_expr(cx.expr_if(sp, condition, then_expr, Option::None)));

    // assertion block
    let block = cx.block(sp, stmts, Option::None);
    let expr_block = cx.expr_block(block);
    MacEager::expr(expr_block)
}

fn create_macro_call(cx: &mut ExtCtxt, sp: Span, name: &str, args: Vec<TokenTree>) -> P<Expr> {
    let macro_path = cx.path(sp, vec!(str_to_ident(name)));
    let invocation = codemap::respan(sp, ast::MacInvocTT(macro_path, args, ast::EMPTY_CTXT));
    cx.expr(sp, ast::ExprMac(invocation))
}

struct Expression {
    column_offset: usize,
    var_name: String
}

struct ExpressionCollector {
    intermediate_counter: usize,
    statements: Vec<P<Stmt>>,
    expressions: Vec<Expression>
}

impl ExpressionCollector {
    fn new() -> ExpressionCollector {
        ExpressionCollector {intermediate_counter: 0, statements: Vec::new(), expressions: Vec::new()}
    }

    fn collect_expression(&mut self, expr: &P<Expr>, cx: &mut ExtCtxt) -> P<Expr> {
        match (*expr).node {
            Expr_::ExprBinary(ref op, ref a, ref b) => {
                let new_a = self.collect_expression(a, cx);
                let new_b = self.collect_expression(b, cx);
                let new_expr = cx.expr_binary(expr.span, op.node, new_a, new_b);
                self.create_let_statement(&new_expr, cx, op.span)
            }
            Expr_::ExprUnary(ref op, ref a) => {
                let new_a = self.collect_expression(a, cx);
                let new_expr = cx.expr_unary(expr.span, *op, new_a);
                self.create_let_statement(&new_expr, cx, expr.span)
            }
            Expr_::ExprLit(_) => {
                // Omit literals since they should be in the original string verbatim
                expr.clone()
            }
            _ => {
                self.create_let_statement(expr, cx, expr.span)
            }
        }
    }

    fn create_let_statement(&mut self, expr: &P<Expr>, cx: &mut ExtCtxt, span: Span) -> P<Expr> {
        let var_name = format!("temp_{}", self.intermediate_counter);
        self.intermediate_counter += 1;
        let ident = str_to_ident(&var_name);
        let let_stmt = cx.stmt_let(expr.span, false, ident, expr.clone());
        let column_offset = cx.codemap().lookup_char_pos(span.lo).col.to_usize();
        self.statements.push(let_stmt);
        self.expressions.push(Expression{column_offset: column_offset, var_name: var_name});
        cx.expr_ident(expr.span, ident)
    }

}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("passert", expand_passert);
}
