use std::{collections::HashMap, hash::Hash};

#[derive(Clone)]
struct State<T: Copy + Eq + Hash> {
    len: usize,              // length of longest string in this state
    link: isize,             // suffix link
    next: HashMap<T, usize>, // transitions

    // Two largest end positions of substrings represented by this state:
    // best1 = largest end position (may be current)
    // best2 = second largest end position (always earlier if exists)
    best1: isize,
    best2: isize,
}

impl<T: Copy + Eq + Hash> State<T> {
    fn new() -> Self {
        Self {
            len: 0,
            link: -1,
            next: HashMap::new(),
            best1: -1,
            best2: -1,
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

    // Insert a position into (best1, best2)
    fn insert_pos(best1: &mut isize, best2: &mut isize, pos: isize) {
        if pos > *best1 {
            *best2 = *best1;
            *best1 = pos;
        } else if pos > *best2 && pos != *best1 {
            *best2 = pos;
        }
    }

    /// Extend SAM with character c at index pos.
    /// Returns (length, after_pos_of_last_earlier_occurrence).
    fn extend(&mut self, c: T, pos: usize) -> (usize, Option<usize>) {
        let cur = self.st.len();
        self.st.push(State {
            len: self.st[self.last].len + 1,
            link: 0,
            next: HashMap::new(),
            best1: pos as isize,
            best2: -1,
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
            // Take one mutable reference to the state, then split fields
            let (b1, b2) = {
                let st = &mut self.st[pu];
                (&mut st.best1, &mut st.best2)
            };
            Self::insert_pos(b1, b2, pos as isize);
            p2 = self.st[pu].link;
        }

        // Query: longest suffix that occurs earlier
        let v = self.st[cur].link as usize;
        let len = self.st[v].len;

        if len == 0 {
            return (0, None);
        }

        let end1 = self.st[v].best1;
        let end2 = self.st[v].best2;

        let end = if end1 < pos as isize { end1 } else { end2 };

        if end < 0 {
            return (0, None);
        }

        // return the position AFTER the earlier occurrence
        let after = end as usize + 1;
        (len, Some(after))
    }

    pub fn push(&mut self, c: T) -> T {
        let (_, after) = self.extend(c, self.items.len());
        self.items.push(c);
        self.index_of_next = after.unwrap_or(0);
        self.predict()
    }

    pub fn predict(&self) -> T {
        self.items[self.index_of_next]
    }
}
