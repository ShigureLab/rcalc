use crate::ast::{Atom, BinaryArithmetic, BinaryOp, Expr, FunctionCall, UnaryArithmetic, UnaryOp};

peg::parser! {
    pub grammar calc_parser() for str {
        use super::Expr;

        pub rule program() -> Expr
            = __ e:expr() __ { e }

        #[cache_left_rec]
        pub rule expr() -> Expr
            = a:expr() _ "+" _ b:term() { BinaryArithmetic::new(BinaryOp::Add, a, b).into() }
            / a:expr() _ "-" _ b:term() { BinaryArithmetic::new(BinaryOp::Sub, a, b).into() }
            / term()

        #[cache_left_rec]
        pub rule term() -> Expr
            = a:term() _ "*" _ b:factor_with_unary_op() { BinaryArithmetic::new(BinaryOp::Mul, a, b).into() }
            / a:term() _ "/" _ b:factor_with_unary_op() { BinaryArithmetic::new(BinaryOp::Div, a, b).into() }
            / factor_with_unary_op()

        #[cache_left_rec]
        pub rule factor_with_unary_op() -> Expr
            = "+" _ a:factor_with_unary_op() { UnaryArithmetic::new(UnaryOp::Pos, a).into() }
            / "-" _ a:factor_with_unary_op() { UnaryArithmetic::new(UnaryOp::Neg, a).into() }
            / a:factor_with_unary_op() "!" { UnaryArithmetic::new(UnaryOp::Fac, a).into() }
            / factor()

        #[cache]
        pub rule factor() -> Expr
            = number()
            / function_call()
            / identifier()
            / "(" _ e:expr() _ ")" { e }

        pub rule function_call() -> Expr
            = id:identifier() _ v:bracketed(<commasep(<expr()>)>) {
                FunctionCall::new (
                    if let Expr::Atom(Atom::Ident(x)) = id { x } else { "".to_owned() },
                    v
                ).into()
            }

        pub rule number() -> Expr
            = n:$("-"? ("0" / ['1'..='9']['0'..='9']*) ("." ['0'..='9']+)?) { Atom::Number(n.parse::<f64>().unwrap()).into() }

        pub rule identifier() -> Expr
            = id:$(['a'..='z' | 'A'..='Z' | '_'] ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { Atom::Ident(id.to_owned()).into() }

        rule commasep<T>(x: rule<T>) -> Vec<T> = v:(x() ** ( _ "," _ ) ) ","? { v }
        rule bracketed<T>(x: rule<T>) -> T = "(" _  v:x() _ ")" { v }
        rule _ = " "*
        rule __ = (" " / "\n" / "\r")*
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expr() {
        assert_eq!(
            calc_parser::expr("1 + 9 / 10"),
            Ok(BinaryArithmetic::new(
                BinaryOp::Add,
                Atom::Number(1_f64).into(),
                BinaryArithmetic::new(
                    BinaryOp::Div,
                    Atom::Number(9_f64).into(),
                    Atom::Number(10_f64).into(),
                )
                .into()
            )
            .into())
        );
        assert_eq!(
            calc_parser::expr("(1 + 9) / 10"),
            Ok(BinaryArithmetic::new(
                BinaryOp::Div,
                BinaryArithmetic::new(
                    BinaryOp::Add,
                    Atom::Number(1_f64).into(),
                    Atom::Number(9_f64).into(),
                )
                .into(),
                Atom::Number(10_f64).into(),
            )
            .into())
        );
    }

    #[test]
    fn term() {
        assert_eq!(
            calc_parser::term("1 * 9 / 10"),
            Ok(BinaryArithmetic::new(
                BinaryOp::Div,
                BinaryArithmetic::new(
                    BinaryOp::Mul,
                    Atom::Number(1_f64).into(),
                    Atom::Number(9_f64).into(),
                )
                .into(),
                Atom::Number(10_f64).into(),
            )
            .into())
        );
        assert_eq!(
            calc_parser::term("a * 9"),
            Ok(BinaryArithmetic::new(
                BinaryOp::Mul,
                Atom::Ident("a".into()).into(),
                Atom::Number(9_f64).into(),
            )
            .into())
        );
        assert_eq!(
            calc_parser::term("9 * a"),
            Ok(BinaryArithmetic::new(
                BinaryOp::Mul,
                Atom::Number(9_f64).into(),
                Atom::Ident("a".into()).into(),
            )
            .into())
        );
    }

    #[test]
    fn factor_with_unary_op() {
        assert_eq!(
            calc_parser::factor_with_unary_op("-f(-x)"),
            Ok(UnaryArithmetic::new(
                UnaryOp::Neg,
                FunctionCall::new(
                    "f".into(),
                    vec![UnaryArithmetic::new(UnaryOp::Neg, Atom::Ident("x".into()).into()).into()]
                )
                .into()
            )
            .into())
        );
        assert_eq!(
            calc_parser::factor_with_unary_op("f(x!)!"),
            Ok(UnaryArithmetic::new(
                UnaryOp::Fac,
                FunctionCall::new(
                    "f".into(),
                    vec![UnaryArithmetic::new(UnaryOp::Fac, Atom::Ident("x".into()).into()).into()]
                )
                .into()
            )
            .into())
        );
    }

    #[test]
    fn factor() {
        assert_eq!(
            calc_parser::factor("f(x)"),
            Ok(FunctionCall::new("f".into(), vec![Atom::Ident("x".into()).into()]).into())
        );
        assert_eq!(calc_parser::factor("0"), Ok(Atom::Number(0 as f64).into()));
        assert_eq!(calc_parser::factor("o"), Ok(Atom::Ident("o".into()).into()));
        assert_eq!(
            calc_parser::factor("(1+2)"),
            Ok(BinaryArithmetic::new(
                BinaryOp::Add,
                Atom::Number(1_f64).into(),
                Atom::Number(2_f64).into(),
            )
            .into(),)
        );
    }

    #[test]
    fn function_call() {
        assert_eq!(
            calc_parser::function_call("f(x)"),
            Ok(FunctionCall::new("f".into(), vec![Atom::Ident("x".into()).into()]).into())
        );
        assert_eq!(
            calc_parser::function_call("f ( x )"),
            Ok(FunctionCall::new("f".into(), vec![Atom::Ident("x".into()).into()]).into())
        );
        assert_eq!(
            calc_parser::function_call("log(2, 4)"),
            Ok(FunctionCall::new(
                "log".into(),
                vec![Atom::Number(2_f64).into(), Atom::Number(4_f64).into()]
            )
            .into())
        );
        assert!(calc_parser::function_call("f").is_err());
        assert!(calc_parser::function_call("f(").is_err());
    }

    #[test]
    fn number() {
        assert_eq!(calc_parser::number("0"), Ok(Atom::Number(0 as f64).into()));
        assert_eq!(calc_parser::number("0.1"), Ok(Atom::Number(0.1_f64).into()));
        assert_eq!(
            calc_parser::number("2.333"),
            Ok(Atom::Number(2.333_f64).into())
        );
        assert_eq!(calc_parser::number("42"), Ok(Atom::Number(42_f64).into()));
        assert!(calc_parser::number("00").is_err());
        assert!(calc_parser::number("o").is_err());
    }

    #[test]
    fn identifier() {
        assert_eq!(
            calc_parser::identifier("o"),
            Ok(Atom::Ident("o".into()).into())
        );
        assert_eq!(
            calc_parser::identifier("_parser"),
            Ok(Atom::Ident("_parser".into()).into())
        );
        assert_eq!(
            calc_parser::identifier("par_123"),
            Ok(Atom::Ident("par_123".into()).into())
        );
        assert_eq!(
            calc_parser::identifier("_123"),
            Ok(Atom::Ident("_123".into()).into())
        );
        assert!(calc_parser::identifier("123").is_err());
    }
}
