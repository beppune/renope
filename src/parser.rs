//
// regex   ::= alt
// alt     ::= concat ('|' concat)*
// concat  ::= repeat+
// repeat  ::= atom ('*' | '+' | '?')?
// atom    ::= literal | '.' | '(' regex ')'

#[derive(Debug,PartialEq)]
enum Ast {
    Literal(char),
    Concat(Box<Ast>, Box<Ast>),
    Alt(Box<Ast>, Box<Ast>),
    Star(Box<Ast>),
    Plus(Box<Ast>),
    Optional(Box<Ast>),
    Stop(&'static str),
}

type AstRes = Result<Ast,&'static str>;

impl Ast {
    fn repeat(r:char, ast:Self) -> AstRes {
        match r {
            '*' => Ok( Star(Box::new(ast)) ),
            '+' => Ok( Plus(Box::new(ast)) ),
            '?' => Ok( Optional(Box::new(ast)) ),
            _ => Err("Expected one of repeat operators: *, +, ?"),
        }
    }
}

use self::Ast::*;

type Input<'a> = std::iter::Peekable<std::str::Chars<'a>>;

// An atom is a Literal o a quantified Literal
fn parse_atom(input: &mut Input) -> Option<Ast> {

    let is_quantifier = |x:char| {
        return x == '*' || x == '+' || x == '?';
    };

    let is_atom = |x:char| {
        return x.is_alphabetic();
    };

    let c:char = input.next()?;

    let ast:Ast;

    if !is_atom(c) {
        ast =  Stop("Expected atom");
        return Some(ast);
    }

    let p = input.peek();

    if let Some(&r) = p {
        if is_quantifier(r) {
            ast = Ast::repeat(r, Literal(c)).unwrap();
            return Some(ast);
        }
    }

    ast = Literal(c);
    Some(ast)
}

fn parse_concat(input: &mut Input) -> Option<Ast> {

    let mut repeat = vec![];

    while let Some(ast) = parse_atom(input) {
        repeat.push( ast);
    }

    let mut it = repeat.into_iter();
    let mut ast:Ast = it.next()?;
    while let Some(tsa) = it.next() {
        ast = Concat( Box::new(ast), Box::new(tsa) );
    }

    Some(ast)
}

fn parse_alt(input: &mut Input) -> Option<Ast> {

    let mut repeat = vec![];

    while let Some(&a) = input.peek() {
        if a == '|' {
            let _ = input.next();
        } else {
            if let Some(ast) = parse_concat(input) {
                repeat.push( ast );
            }
        }
    }

    let mut it = repeat.into_iter();
    let mut ast = it.next()?;
    while let Some(tsa) = it.next() {
        ast = Alt( Box::new(ast), Box::new(tsa) );
    }

    Some(ast)

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_atom() {
        let mut it = "a?=".chars().peekable();

        let mut ast = parse_atom(&mut it);
        assert_eq!(Some(Ast::Optional(Box::new(Ast::Literal('a')))), ast); 

        ast = parse_atom(&mut it);
        assert_eq!(Some(Ast::Stop("Expected atom")), ast); 

    }

}
