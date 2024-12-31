#![feature(rustc_private)]
#![warn(unused_extern_crates)]

// extern crate rustc_arena;
// extern crate rustc_ast;
// extern crate rustc_ast_pretty;
// extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_hir;
// extern crate rustc_hir_pretty;
// extern crate rustc_index;
// extern crate rustc_infer;
// extern crate rustc_lexer;
extern crate rustc_middle;
// extern crate rustc_mir_dataflow;
// extern crate rustc_parse;
extern crate rustc_span;
// extern crate rustc_target;
// extern crate rustc_trait_selection;

use clippy_utils::{diagnostics::span_lint_and_help, is_diag_trait_item, source::snippet};
use rustc_errors::Applicability;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::lint::in_external_macro;
use rustc_span::Symbol;

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Known problems
    /// Remove if none.
    ///
    /// ### Example
    /// ```rust
    /// // example code where a warning is issued
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// ```
    pub STATE_SCOPED_ENTITIES,
    Warn,
    "checks for spawning entities without a state scope"
}

impl StateScopedEntities {
    //returns true if
    fn argument_contains_state_scope<'tcx>(ctx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) -> bool {
        if let ExprKind::Tup(items) = expr.kind {
            return items
                .iter()
                .any(|item| Self::argument_contains_state_scope(ctx, item));
        }
        let rustc_middle::ty::TyKind::Adt(adt, _) =
            ctx.typeck_results().expr_ty(expr).peel_refs().kind()
        else {
            return false;
        };

        let Some(variant) = adt.variants().iter().next() else {
            return false;
        };

        variant.ident(ctx.tcx).name == Symbol::intern("StateScoped")
    }
}

impl<'tcx> LateLintPass<'tcx> for StateScopedEntities {
    // A list of things you might check can be found here:
    // https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html

    fn check_expr(&mut self, ctx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        if in_external_macro(ctx.tcx.sess, expr.span) {
            return;
        }

        let ExprKind::MethodCall(segment, object, func_args, expr_span) = expr.kind else {
            return;
        };

        if segment.ident.name != Symbol::intern("spawn") {
            return;
        }

        let rustc_middle::ty::TyKind::Adt(adt, _) =
            ctx.typeck_results().expr_ty(object).peel_refs().kind()
        else {
            return;
        };

        let Some(variant) = adt.variants().iter().next() else {
            return;
        };

        //only need to check for .spawn() on Commands and not ChildBuilder since the despawn is recursive
        if variant.ident(ctx.tcx).name != Symbol::intern("Commands") {
            return;
        }

        if !func_args
            .iter()
            .any(|arg| StateScopedEntities::argument_contains_state_scope(ctx, arg))
        {
            span_lint_and_help(
                ctx,
                STATE_SCOPED_ENTITIES,
                expr_span,
                "Commands::spawn() call is missing StateScoped component",
                None,
                "add a StateScoped() component",
            );
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
