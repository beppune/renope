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

    let mut ast:Ast;

    if !is_atom( *(input.peek()?) ) {
        return None;
    }

    let c = input.next()?;
    ast = Literal(c);

    if let Some(&q) = input.peek() {
        match q {
            q if is_quantifier(q) => {
                ast = Ast::repeat(q, ast).expect("Repeat");
                input.next();
            },
            _ => {}
        }
    }

    return Some(ast);
}

fn parse_concat(input: &mut Input) -> Option<Ast> {
    let mut atoms = vec![];

    while let Some(ast) = parse_atom(input) {
        if let Some(&f) = input.peek() {
            if !is_atom(f) {
                atoms.push( Stop("atom") );
            }
        }
        atoms.push( ast );
    }

    let mut it = atoms.into_iter();
    let mut ast = it.next()?;
    while let Some(tsa) = it.next() {
        ast = Concat( Box::new(ast), Box::new(tsa) );
    }

    Some(ast)
}

fn parse_alt(input: &mut Input) -> Option<Ast> {
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_it() {
        let alt = Alt( Box::new(Literal('a')), Box::new(Literal('f')) );
        let concat = Concat( Box::new(alt), Box::new(Star(Box::new(Literal('d')))) );

        println!("{concat}");

    }

    #[test]
    #[ignore]
    fn test_atom() {
        let mut it = "abc+=fg+".chars().peekable();

        let mut atom = parse_atom(&mut it);
        if let Some(ast) = atom {
            println!("{ast}");
        }

        atom = parse_atom(&mut it);
        if let Some(ast) = atom {
            println!("{ast}");
        }

        atom = parse_atom(&mut it);
        if let Some(ast) = atom {
            println!("{ast}");
        }

        atom = parse_atom(&mut it);
        if let Some(ast) = atom {
            println!("{ast}");
        }
    }

}
