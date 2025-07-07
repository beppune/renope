//
// regex   ::= alt
// alt     ::= concat ('|' concat)*
// concat  ::= repeat+
// repeat  ::= atom ('*' | '+' | '?')?
// atom    ::= literal | '.' | '(' regex ')'

enum Ast {
    Empty,
    Literal(char),
    Concat(Box<Ast>, Box<Ast>),
    Alt(Box<Ast>, Box<Ast>),
    Star(Box<Ast>),
}

type Result = std::result::Result<Ast, &'static str>;

fn parse_alt(input: &mut std::iter::Peekable<std::str::Chars<>>) -> Result {


    let mut p = input.peek();
    let ast = match p {
        Some(c) => match c {
            c if c.is_alphabetic() => {

            }
        },
        _ =>  Ast::Empty
    };

    Ok(ast)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_this() {
        let input = "ab";
        assert_eq!(true, true);
    }
}
