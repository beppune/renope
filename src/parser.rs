//
// regex   ::= alt
// alt     ::= concat ('|' concat)*
// concat  ::= repeat+
// repeat  ::= atom ('*' | '+' | '?')?
// atom    ::= literal | '.' | '(' regex ')'
use std::fmt::Display;

#[derive(Debug,PartialEq)]
enum Ast {
    Literal(char),
    Concat(Box<Ast>, Box<Ast>),
    Alt(Box<Ast>, Box<Ast>),
    Star(Box<Ast>),
    Plus(Box<Ast>),
    Optional(Box<Ast>),
    Stop(&'static str),
    Group(Box<Ast>),
}

struct AstPreOrderIter<'a> {
    stack: Vec<&'a Ast>,
}

impl<'a> AstPreOrderIter<'a> {
    fn new(ast:&'a Ast) -> Self {
        Self {
            stack: vec![ast],
        }
    }
}

impl<'a> Iterator for AstPreOrderIter<'a> {
    type Item = &'a Ast;

    fn next(&mut self) -> Option<Self::Item> {
        let node:&'a Ast = self.stack.pop()?;
        
        match node {
            Concat(ast, tsa) | Alt(ast, tsa) => {
                self.stack.push( &**ast);
                self.stack.push( &**tsa);
            },

            Star(ast) | Plus(ast) | Optional(ast) => {
                self.stack.push( &**ast );
            },

            _ => {}
        }

        Some(node)

    }
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

    fn preorder_iter(& self) -> AstPreOrderIter<'_> {
        AstPreOrderIter::new(self)
    }
}

impl Display for Ast {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match &self {
        Literal(c) => write!(f, "{c}"),
        Concat(ast, tsa) => write!(f, "{}{}", **ast, **tsa),
        Alt(ast, tsa) => write!(f, "{}|{}", **ast, **tsa),
        Star(ast) => write!(f, "{}*", **ast),
        Plus(ast) => write!(f, "{}+", **ast),
        Optional(ast) => write!(f, "{}?", **ast),
        Stop(e) => write!(f, "<{e}>"),
        Group(ast) => write!(f, "{}?", **ast),
    }
   }
}

use self::Ast::*;

type Input<'a> = std::iter::Peekable<std::str::Chars<'a>>;

// An atom is a Literal o a quantified Literal
fn parse_atom(input: &mut Input) -> Option<Ast> {
    let p = input.peek()?;
    return Some(Literal(*p));
}

fn parse_concat(input: &mut Input) -> Option<Ast> {
    return parse_atom(input);
}

fn parse_alt(input: &mut Input) -> Option<Ast> {
    return parse_concat(input);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_group() {
        let mut ast = parse_alt( &mut "".chars().peekable() );
        assert_eq!( None, ast );

        ast = parse_alt( &mut "b".chars().peekable() );
        assert_eq!( Some(Literal('b')), ast);
        // let exp = Group(Box::new(Literal('b')));
    }

    #[test]
    #[ignore]
    fn test_alt() {
        let mut it = "as|w?|w*".chars().peekable();
        let ast = parse_alt(&mut it);

        dbg!(ast);

    }


    #[test]
    #[ignore]
    fn test_concat() {
        let mut it = "ab+=c*d?".chars().peekable();
        let ast = parse_concat(&mut it).unwrap();
        println!("\n### {ast} ###");
        dbg!(ast);
    }

    #[test]
    #[ignore]
    fn test_atom() {
        let mut it = "".chars().peekable();
        let ast = parse_atom(&mut it);
        assert_eq!(None, ast);

        let mut it = "ab+".chars().peekable();
        let ast = parse_atom(&mut it);
        assert_eq!( Some(Literal('a')), ast );
        let ast = parse_atom(&mut it);
        assert_eq!( Some(Plus(Box::new(Literal('b')))), ast);
        let ast = parse_atom(&mut it);
        assert_eq!( None, ast);

        it = "=?".chars().peekable();
        let ast = parse_atom(&mut it);
        assert_eq!( None, ast );
        assert_eq!( Some(&'='), it.peek() );

        it = "|alt".chars().peekable();
        let ast = parse_atom(&mut it);
        dbg!(ast);
    }

}
