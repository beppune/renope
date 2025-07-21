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
   fn fmt(&self, f: &mut fmt::Formatter<'_'>) -> fmt::Result {
       match &self {
        Literal(c) | Stop(c) => write!(f, "{}", c),
        Concat(ast, tsa) | Alt(ast,tsa) => write!(f, "{}{}", Display::fmt(ast,f), Display::fmt(tsa,f)),

        Star(ast) => write!("{}*", Display::fmt(ast,f)),
        Plus(ast) => write!("{}+", Display::fmt(ast,f)),
        Optional(ast) => write!("{}?", Display::fmt(ast,f)),
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

    let c = input.peek()?;

    let ast:Ast;

    if !is_atom(*c) {
        ast = Stop( "Expected atom character" );
        return Some(ast);
    }

    ast = Literal(*c);
    let _ = input.next();
    return Some(ast);
}

fn parse_concat(input: &mut Input) -> Option<Ast> {
    None
}

fn parse_alt(input: &mut Input) -> Option<Ast> {
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it() {
        let ast = Concat( Box::new( 
                Concat( Box::new(Literal('8')), Box::new(Literal('r') ) )
        ), Box::new( Literal('d' ) ) );

        for a in ast.preorder_iter() {
            dbg!(&a);
        }

    }

    #[test]
    #[ignore]
    fn test_atom() {
        let mut it = "ab=c+".chars().peekable();

        while let Some(ast) = parse_atom(&mut it) {
            if let Stop(msg) = ast {
                println!("Error {}", msg);
                break;
            }
            dbg!(ast);
        }

    }

}
