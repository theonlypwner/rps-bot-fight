use strum::EnumCount;

use crate::bot::Move;

#[derive(Clone)]
struct State {
    len: usize,                 // length of longest string in this state
    link: isize,                // suffix link
    next: [isize; Move::COUNT], // transitions

    // Two largest end positions of substrings represented by this state:
    // best1 = one past largest end position (may be current)
    // best2 = one past second largest end position (always earlier if exists)
    best1: usize,
    best2: usize,
}

impl State {
    fn new() -> Self {
        Self {
            len: 0,
            link: -1,
            next: [-1; Move::COUNT],
            best1: 0,
            best2: 0,
        }
    }
}

pub struct SuffixAutomaton {
    st: Vec<State>,
    last: usize,

    items: Vec<Move>,
    index_of_next: usize,
}

impl SuffixAutomaton {
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
    pub fn push(&mut self, c: Move) {
        self.items.push(c);
        let pos = self.items.len();

        let cur = self.st.len();
        self.st.push({
            let mut s = State::new();
            s.len = self.st[self.last].len + 1;
            s.best1 = pos;
            s
        });

        let mut p = self.last as isize;
        while p != -1 && self.st[p as usize].next[c as usize] == -1 {
            self.st[p as usize].next[c as usize] = cur as isize;
            p = self.st[p as usize].link;
        }

        if p == -1 {
            self.st[cur].link = 0;
        } else {
            let q = self.st[p as usize].next[c as usize];
            if self.st[p as usize].len + 1 == self.st[q as usize].len {
                self.st[cur].link = q as isize;
            } else {
                let clone = self.st.len();
                self.st.push(self.st[q as usize].clone());
                self.st[clone].len = self.st[p as usize].len + 1;

                while p != -1 && self.st[p as usize].next[c as usize] == q {
                    self.st[p as usize].next[c as usize] = clone as isize;
                    p = self.st[p as usize].link;
                }

                self.st[q as usize].link = clone as isize;
                self.st[cur].link = clone as isize;
            }
        }

        self.last = cur;

        // Propagate the new position upward along suffix links
        //
        // All other parts have O(1) amortized time, but this part takes O(n) time,
        // making the overall time complexity similar to
        // building and scanning the LPS or Z array for the reversed string.
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
            p2 = st.link;
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

    pub fn predict(&self) -> Move {
        self.items[self.index_of_next]
    }
}
