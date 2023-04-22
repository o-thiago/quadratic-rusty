use std::{
    char,
    fmt::Display,
    ops::{Add, Neg, Sub},
};

use num::{traits::Pow, Complex, Float, Num, NumCast};

#[derive(Debug)]
pub struct QuadraticRoots<T: Num>(pub T, pub T);

impl<T: Num + Display> Display for QuadraticRoots<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
pub struct SingleRoot<T: Num>(pub T);

impl<T: Num + Display> Display for SingleRoot<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.0)
    }
}

#[derive(Debug)]
pub enum SolvingVariant {
    PositivePart,
    NegativePart,
}

impl SolvingVariant {
    fn get_operation<T: Num>(self) -> impl Fn(T, T) -> <T as Add<T>>::Output {
        match self {
            SolvingVariant::PositivePart => Add::add,
            SolvingVariant::NegativePart => Sub::sub,
        }
    }
}

#[derive(Debug)]
pub enum RealQuadraticRootResult<T: Num + Clone> {
    SingleRoot(SingleRoot<T>),
    RealRoots(QuadraticRoots<T>),
    Complex(QuadraticExpression<Complex<T>>),
}

pub trait QuadraticExpr<R> {
    fn get_roots(&self) -> R;
}

#[derive(Clone, Debug)]
pub struct QuadraticExpression<T: Num> {
    pub a: T,
    pub b: T,
    pub c: T,
}

pub struct DisplayQuadraticExpression<'a, T: Num + PartialOrd + Display + Copy> {
    expr: &'a QuadraticExpression<T>,
    variable: char,
}

impl<'a, T: Num + Display + Copy + PartialOrd> DisplayQuadraticExpression<'a, T> {
    pub fn new(expr: &'a QuadraticExpression<T>, variable: char) -> Self {
        Self { expr, variable }
    }
}

impl<T: Num + Display + Copy + PartialOrd> DisplayQuadraticExpression<'_, T> {
    fn n_with_symbol(&self, n: T) -> String {
        if n >= T::zero() {
            format!("+{n}")
        } else {
            n.to_string()
        }
    }

    pub fn display_part(&self, n: T) -> String {
        format!("{}{}", self.n_with_symbol(n), self.variable)
    }
}

impl<T: Num + Display + Copy + PartialOrd> Display for DisplayQuadraticExpression<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}^2{}{}",
            self.display_part(self.expr.a),
            self.display_part(self.expr.b),
            self.n_with_symbol(self.expr.a)
        )
    }
}

struct ConvertedQuadraticExpression<T: Num>(QuadraticExpression<T>);

impl<T: Num, I: Num + Into<T>> From<QuadraticExpression<I>> for ConvertedQuadraticExpression<T> {
    fn from(val: QuadraticExpression<I>) -> Self {
        ConvertedQuadraticExpression(QuadraticExpression::new(
            val.a.into(),
            val.b.into(),
            val.c.into(),
        ))
    }
}

impl<T: Num> QuadraticExpression<T> {
    pub fn new(a: T, b: T, c: T) -> Self {
        Self { a, b, c }
    }
}

impl<T: Num + NumCast + Neg<Output = T> + Pow<T, Output = T> + Clone + Copy>
    QuadraticExpression<T>
{
    pub fn delta(&self) -> T {
        self.b.pow(T::from(2).unwrap()) - T::from(4).unwrap() * self.a * self.c
    }

    pub fn get_single_root_unchecked(&self, solving: SolvingVariant) -> T {
        solving.get_operation()(-self.b, self.delta().pow(T::from(1 / 2).unwrap()))
            / T::from(2).unwrap()
            * self.a
    }

    pub fn get_all_roots_unchecked(&self) -> QuadraticRoots<T> {
        QuadraticRoots(
            self.get_single_root_unchecked(SolvingVariant::PositivePart),
            self.get_single_root_unchecked(SolvingVariant::NegativePart),
        )
    }
}

impl<T: Float + Pow<T, Output = T> + Into<Complex<T>>> QuadraticExpr<RealQuadraticRootResult<T>>
    for QuadraticExpression<T>
{
    fn get_roots(&self) -> RealQuadraticRootResult<T> {
        let zero = T::zero();

        match self.delta() {
            d if d > zero => RealQuadraticRootResult::RealRoots(self.get_all_roots_unchecked()),
            d if d == zero => RealQuadraticRootResult::SingleRoot(
                SingleRoot(
                self.get_single_root_unchecked(SolvingVariant::PositivePart)),
            ),
            _ => RealQuadraticRootResult::Complex(
                ConvertedQuadraticExpression::from(self.to_owned()).0,
            ),
        }
    }
}

impl<T: Float> QuadraticExpr<QuadraticRoots<Complex<T>>> for QuadraticExpression<Complex<T>> {
    fn get_roots(&self) -> QuadraticRoots<Complex<T>> {
        self.get_all_roots_unchecked()
    }
}
