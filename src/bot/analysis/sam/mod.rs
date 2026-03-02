use std::{collections::HashMap, hash::Hash};

#[derive(Clone)]
struct State<T: Copy + Eq + Hash> {
    len: usize,              // length of longest string in this state
    link: isize,             // suffix link
    next: HashMap<T, usize>, // transitions

    // Two largest end positions of substrings represented by this state:
    // best1 = one past largest end position (may be current)
    // best2 = one past second largest end position (always earlier if exists)
    best1: usize,
    best2: usize,
}

impl<T: Copy + Eq + Hash> State<T> {
    fn new() -> Self {
        Self {
            len: 0,
            link: -1,
            next: HashMap::new(),
            best1: 0,
            best2: 0,
        }
    }
}

pub struct SuffixAutomaton<T: Copy + Eq + Hash> {
    st: Vec<State<T>>,
    last: usize,

    items: Vec<T>,
    index_of_next: usize,
}

impl<T: Copy + Eq + Hash> SuffixAutomaton<T> {
    pub fn new() -> Self {
        let mut st = Vec::new();
        st.push(State::new()); // root
        Self {
            st,
            last: 0,
            items: Vec::new(),
            index_of_next: 0,
        }
    }

    /// Extend SAM with character c.
    pub fn push(&mut self, c: T) {
        self.items.push(c);
        let pos = self.items.len();

        let cur = self.st.len();
        self.st.push(State {
            len: self.st[self.last].len + 1,
            link: 0,
            next: HashMap::new(),
            best1: pos,
            best2: 0,
        });

        let mut p = self.last as isize;
        while p != -1 && !self.st[p as usize].next.contains_key(&c) {
            self.st[p as usize].next.insert(c, cur);
            p = self.st[p as usize].link;
        }

        if p == -1 {
            self.st[cur].link = 0;
        } else {
            let q = self.st[p as usize].next[&c];
            if self.st[p as usize].len + 1 == self.st[q].len {
                self.st[cur].link = q as isize;
            } else {
                let clone = self.st.len();
                self.st.push(self.st[q].clone());
                self.st[clone].len = self.st[p as usize].len + 1;

                while p != -1 && self.st[p as usize].next.get(&c) == Some(&q) {
                    self.st[p as usize].next.insert(c, clone);
                    p = self.st[p as usize].link;
                }

                self.st[q].link = clone as isize;
                self.st[cur].link = clone as isize;
            }
        }

        self.last = cur;

        // Propagate the new position upward along suffix links.
        let mut p2 = self.st[cur].link;
        while p2 != -1 {
            let pu = p2 as usize;
            let st = &mut self.st[pu];
            // Insert a position into (best1, best2)
            if pos > st.best1 {
                st.best2 = st.best1;
                st.best1 = pos;
            } else if pos > st.best2 && pos != st.best1 {
                st.best2 = pos;
            }
            p2 = self.st[pu].link;
        }

        self.index_of_next = {
            // Query: longest suffix that occurs earlier
            let v = self.st[cur].link as usize;
            let len = self.st[v].len;

            if len == 0 {
                0
            } else {
                let end1 = self.st[v].best1;
                let end2 = self.st[v].best2;

                if end1 < pos { end1 } else { end2 }
            }
        };
    }

    pub fn predict(&self) -> T {
        self.items[self.index_of_next]
    }
}
