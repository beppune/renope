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

fn parse_concat(input: &mut PeekChars) -> ResultAst {    

    let mut ls = vec![];

    while let Some(c) = input.peek() {
        if !c.is_alphabetic() {
            break;
        }

        ls.push( Ast::Literal(*c) );
        input.next();
    }

    let mut ast = Ast::Empty;
    for c in ls.into_iter() {
        ast = Ast::Concat( Box::new(ast), Box::new(c) );
    }

    Ok(ast)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cat() {

        let ast = parse_concat(&mut "abcd".chars().peekable());

        dbg!(ast);

        assert_eq!(true, true); 
    }
}
