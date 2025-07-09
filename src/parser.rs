//
// regex   ::= alt
// alt     ::= concat ('|' concat)*
// concat  ::= repeat+
// repeat  ::= atom ('*' | '+' | '?')?
// atom    ::= literal | '.' | '(' regex ')'

#[derive(Debug,PartialEq)]
enum Ast {
    Empty,
    Literal(char),
    Concat(Box<Ast>, Box<Ast>),
    Alt(Box<Ast>, Box<Ast>),
}

type ResultAst = std::result::Result<Ast, &'static str>;

type PeekChars<'a> = std::iter::Peekable<std::str::Chars<'a>>;

fn parse_literal(input: &mut PeekChars) -> ResultAst {

    let p = input.next();

    match p {
        Some(c) if c.is_alphabetic() => Ok(Ast::Literal(c)),
        None => Ok(Ast::Empty),
        _ => Err("literal expected"),
    }

}

fn parse_concat(input: &mut PeekChars) -> ResultAst {
    

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_this() {
        let res = parse_literal(&mut "a".chars().peekable());
        assert_eq!(res.unwrap(), Ast::Literal('a') );

        let empty = parse_literal(&mut "".chars().peekable());
        assert_eq!(empty.unwrap(), Ast::Empty );

        let error = parse_literal(&mut "!".chars().peekable());
        assert_eq!(error, Err("literal expected") );
    }
}
