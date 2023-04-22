use quadratic_expression::{DisplayQuadraticExpression, RealQuadraticRootResult};
use text_io::{read, try_read};

use crate::quadratic_expression::{QuadraticExpr, QuadraticExpression};

pub mod quadratic_expression;

fn get_variable_from_user(var: char) -> f64 {
    print!("Valor de ({var}): ");
    let input: Result<f64, _> = try_read!();

    match input {
        Ok(input) => input,
        Err(..) => {
            println!("Isto não é um número...");
            get_variable_from_user(var)
        }
    }
}

fn loop_quadratics_from_user() {
    let a = get_variable_from_user('a');
    let b = get_variable_from_user('b');
    let c = get_variable_from_user('c');

    let expr = QuadraticExpression::new(a, b, c);
    let display_expr = DisplayQuadraticExpression::new(&expr, 'x');

    match expr.get_roots() {
        RealQuadraticRootResult::SingleRoot(root) => {
            println!("{display_expr} possui uma raiz: {root}")
        }
        RealQuadraticRootResult::RealRoots(roots) => {
            println!("{display_expr} possui duas raizes: {roots}")
        }
        RealQuadraticRootResult::Complex(complex) => {
            let roots = complex.get_roots();

            println!("{display_expr} possui as seguintes raizes complexas: {roots}",)
        }
    };

    print!("Deseja saber o valor de outra equação de segunda grau? (S/N): ");
    let response: String = read!();

    if response.to_lowercase() == "s" {
        loop_quadratics_from_user()
    }
}

fn main() {
    println!("Irei lhe ajudar a fazer equações de segundo grau...");

    loop_quadratics_from_user()
}
