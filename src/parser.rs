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
    Star(Box<Ast>),
    Plus(Box<Ast>),
    Optional(Box<Ast>),
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

fn parse_repeat(input: &mut Input) -> Option<Ast> {
    let c:char = input.next()?;

    let ast:Ast;

    let repeat = input.peek();
    match repeat {
        None => ast = Literal(c),
        Some(&r) => {
            ast = match r {
                '*' | '+' | '?' => {
                    let _ = input.next();
                    Ast::repeat(r, Literal(c))
                        .expect("PARSE_REPEAT: expected one of the repeat chars: *,+,?")
                }
                _ => Literal(c),
            };
        },
    }

    Some(ast)

}

fn parse_concat(input: &mut Input) -> Option<Ast> {

    let mut repeat = vec![];

    while let Some(ast) = parse_repeat(input) {
        repeat.push( ast);
    }

    let mut it = repeat.into_iter();
    let mut ast:Ast = it.next()?;
    while let Some(tsa) = it.next() {
        ast = Concat( Box::new(ast), Box::new(tsa) );
    }
    // dbg!(&ast);
    //
    // let mut n = it.next()?;
    // dbg!(&n);
    //
    // n = it.next()?;
    // dbg!(&n);
    //
    // n = it.next()?;
    // dbg!(&n);
    //
    // n = it.next()?;
    // dbg!(&n);
    //
    Some(ast)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tes_concat() {
        let s =  "ab+c*d?"; 
        println!("{s}");

        let mut it = s.chars().peekable();

        let ast = parse_concat(&mut it);
        dbg!(ast);
    }

    #[test]
    #[ignore]
    fn test_char() {
        let s =  "ab+c*d?"; 
        println!("{s}");

        let mut it = s.chars().peekable();

        let mut ast = parse_repeat(&mut it);
        dbg!(ast);

        ast = parse_repeat(&mut it);
        dbg!(ast);

        ast = parse_repeat(&mut it);
        dbg!(ast);

        ast = parse_repeat(&mut it);
        dbg!(ast);
    }

}
