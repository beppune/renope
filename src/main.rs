use std::collections::{HashMap, HashSet};

struct Nfa{
}

impl Nfa{
    fn new() -> Self {
        Self {}
    }

    fn state(s:u32, i:char, to:&Vec<u32>) {
        unimplemented!();
    }

    fn finals(s:&Vec<u32>) {
        unimplemented!();
    }

    fn start(s:u32) {
        unimplemented!();
    }
}


fn main() {
    let mut set = HashSet::new();

    assert_eq!(set.insert(1), true);
    assert_eq!(set.insert(1), false);

    let mut state: HashMap<char, HashSet<u32>> = HashMap::new();

    assert_eq!(state.insert('a', set), None);

    let mut nfa = Nfa::new();

    // a(a|b)*
    nfa.state(0, 'a', vec![1]);
    nfa.state(1, '-', vec![2]);
    nfa.state(2, '-', vec![3,9]);
    nfa.state(3, '-', vec![4,6]);
    nfa.state(4, 'a', vec![5]);
    nfa.state(5, '-', vec![8]);
    nfa.state(6, 'b', vec![7]);
    nfa.state(7, 'a', vec![8]);
    nfa.state(8, '-', vec![3,9]);

    nfa.finals(vec![9]);
    nfa.start(0);

}
