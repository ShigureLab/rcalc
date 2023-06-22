use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[clap(name = "rcalc")]
#[clap(author = "Nyakku Shigure <sigure.qaq@gmail.com>")]
#[clap(version = "0.1")]
#[clap(about = "A simple calculator.")]
#[clap(allow_negative_numbers = true)]
pub struct Cli {
    pub expr: String,

    #[clap(short, long)]
    pub jit: bool,

    #[clap(short, long)]
    pub verbose: bool,

    #[clap(short, long)]
    pub precision: Option<usize>,

    #[clap(long)]
    pub pure: bool,

    #[clap(short)]
    pub a: Option<f64>,

    #[clap(short)]
    pub b: Option<f64>,

    #[clap(short)]
    pub c: Option<f64>,

    #[clap(short)]
    pub d: Option<f64>,

    #[clap(short)]
    pub e: Option<f64>,

    #[clap(short)]
    pub x: Option<f64>,

    #[clap(short)]
    pub y: Option<f64>,

    #[clap(short)]
    pub z: Option<f64>,
}

pub fn get_variables(cli: &Cli) -> HashMap<String, f64> {
    let mut variables = HashMap::new();
    if let Some(a) = cli.a {
        variables.insert("a".into(), a);
    }
    if let Some(b) = cli.b {
        variables.insert("b".into(), b);
    }
    if let Some(c) = cli.c {
        variables.insert("c".into(), c);
    }
    if let Some(d) = cli.d {
        variables.insert("d".into(), d);
    }
    if let Some(e) = cli.e {
        variables.insert("e".into(), e);
    }
    if let Some(x) = cli.x {
        variables.insert("x".into(), x);
    }
    if let Some(y) = cli.y {
        variables.insert("y".into(), y);
    }
    if let Some(z) = cli.z {
        variables.insert("z".into(), z);
    }
    variables
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::assert_close;

    #[test]
    fn common() {
        let expr = "a * b + 9";
        let cli = Cli::parse_from(["rcalc", expr]);
        assert_eq!(expr, cli.expr);
    }

    #[test]
    fn jit() {
        let expr = "expr";
        let cli = Cli::parse_from(["rcalc", expr]);
        assert!(!cli.jit);

        let cli = Cli::parse_from(["rcalc", expr, "-j"]);
        assert!(cli.jit);

        let cli = Cli::parse_from(["rcalc", expr, "--jit"]);
        assert!(cli.jit);
    }

    #[test]
    fn verbose() {
        let expr = "expr";
        let cli = Cli::parse_from(["rcalc", expr]);
        assert!(!cli.verbose);

        let cli = Cli::parse_from(["rcalc", expr, "-v"]);
        assert!(cli.verbose);

        let cli = Cli::parse_from(["rcalc", expr, "--verbose"]);
        assert!(cli.verbose);
    }

    #[test]
    fn precision() {
        let expr = "expr";
        let cli = Cli::parse_from(["rcalc", expr]);
        assert_eq!(cli.precision, None);

        let cli = Cli::parse_from(["rcalc", expr, "-p", "3"]);
        assert_eq!(cli.precision, Some(3));

        let cli = Cli::parse_from(["rcalc", expr, "-p=5"]);
        assert_eq!(cli.precision, Some(5));

        let cli = Cli::parse_from(["rcalc", expr, "--precision", "7"]);
        assert_eq!(cli.precision, Some(7));

        let cli = Cli::parse_from(["rcalc", expr, "--precision=9"]);
        assert_eq!(cli.precision, Some(9));
    }

    #[test]
    fn pure() {
        let expr = "expr";
        let cli = Cli::parse_from(["rcalc", expr]);
        assert!(!cli.pure);

        let cli = Cli::parse_from(["rcalc", expr, "--pure"]);
        assert!(cli.pure);
    }

    #[test]
    fn variables() {
        let expr = "expr";
        let cli = Cli::parse_from(["rcalc", expr]);
        let variables = get_variables(&cli);
        assert!(variables.is_empty());

        let cli = Cli::parse_from(["rcalc", expr, "-a", "-1.1"]);
        let variables = get_variables(&cli);
        assert_eq!(variables.get("b"), None);
        assert_close(variables["a"], -1.1);

        let cli = Cli::parse_from(["rcalc", expr, "-b=-0.999"]);
        let variables = get_variables(&cli);
        assert_close(variables["b"], -0.999);
    }
}
