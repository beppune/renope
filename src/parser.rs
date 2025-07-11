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

    let mut iter = ls.into_iter();
    let mut ast = iter.next().unwrap_or(Ast::Empty);
    for c in iter {
        ast = Ast::Concat( Box::new(ast), Box::new(c) );
    }

    Ok(ast)
}

fn parse_alt(input: &mut PeekChars) -> ResultAst {

    let mut ast = parse_concat(input)?;

    while input.peek() == Some(&'|') {
        input.next();
        ast = Ast::Alt( Box::new(ast), Box::new(parse_concat(input)?) );
    }

    Ok( ast )
}

fn regex_compile(s:&str) -> ResultAst {
    parse_alt(&mut s.chars().peekable())    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cat() {

        let mut ast = regex_compile("");
        dbg!(ast);

        let mut ast = regex_compile("ab");
        dbg!(ast);

        let mut ast = regex_compile("ab|cd|ef|gh");
        dbg!(ast);

    }
    

}
