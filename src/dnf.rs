use std::{collections::HashMap, fmt};

fn display_vec<T: fmt::Display>(vec: &[T]) -> String {
    let content = vec.iter()
        .map(|item| format!("{}", item))
        .collect::<Vec<_>>()
        .join(" ");
    format!("{{ {content} }}")
}

struct Nfa{
    states: Vec<HashMap<char,Vec<u32>>>,
    final_states: Vec<u32>,
    start_state: u32
}

impl Nfa{
    fn new() -> Self {
        Self {
            states: vec![],
            final_states: vec![],
            start_state: 0
        }
    }

    fn state(&mut self, s:usize, i:Option<char>, to:Vec<u32>) {

        for _ in self.states.len()..=s {
            self.states.push( HashMap::new() );
        }

        let cc = i.unwrap_or('\u{025B}');

        self.states[s] = HashMap::from( [(cc, to)] )

    }

    fn finals(&mut self, s:Vec<u32>) {
        self.final_states = s;
    }

    fn start(&mut self, s:u32) {
        self.start_state = s
    }

    fn not_export(&self) -> String {

        let mut not = String::new();
        not.push_str("digraph {\n\trankdir=\"LR\"\n");

        for (i,m) in self.states.iter().enumerate() {
            for (k, v) in m {
                not.push_str( format!("\t{} -> {} [label=\"{}\"]\n", i, display_vec(v), k).as_str() );
            }
        }

        for i in self.final_states.iter() {
            not.push_str( format!("\n\t{i} [shape=\"doublecircle\"]").as_str() );
        }

        not.push_str( format!("\n\t_ -> {}\n\t_ [style=\"invis\", width=0]\n", self.start_state ).as_str() );
        
        not.push_str("\n}");
        not
    }
}


fn test() {

    let mut nfa = Nfa::new();

    // a(a|b)*
    nfa.state(0, Some('a'), vec![1]);
    nfa.state(1, None, vec![2]);
    nfa.state(2, None, vec![3,9]);
    nfa.state(3, None, vec![4,6]);
    nfa.state(4, Some('a'), vec![5]);
    nfa.state(5, None, vec![8]);
    nfa.state(6, Some('b'), vec![7]);
    nfa.state(7, Some('a'), vec![8]);
    nfa.state(8, None, vec![3,9]);

    nfa.finals(vec![9]);
    nfa.start(0);

    println!("\n{}\n", nfa.not_export() );

}
