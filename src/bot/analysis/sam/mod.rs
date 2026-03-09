use std::mem::swap;

use strum::EnumCount;

use crate::bot::Move;

struct Node<T> {
    left: isize,
    right: isize,
    parent: isize,
    rev: bool,

    has_assign: bool,
    val: T,
}

impl<T: Copy + Default> Node<T> {
    fn new(val: T) -> Self {
        Self {
            left: -1,
            right: -1,
            parent: -1,
            rev: false,

            has_assign: false,
            val,
        }
    }

    fn apply_assign(&mut self, val: T) {
        self.val = val;
        self.has_assign = true;
    }
}

struct LinkCutTree<T> {
    nodes: Vec<Node<T>>,
}

impl<T: Copy + Default> LinkCutTree<T> {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_vertex(&mut self, initial_value: T) {
        self.nodes.push(Node::new(initial_value));
    }

    // Link x as a child of p (assumes no cycle is created)
    pub fn link_parent(&mut self, x: usize, p: usize) {
        self.make_root(x);
        self.nodes[x].parent = p as isize;
    }

    // Cut x from its parent, if any
    pub fn cut_parent(&mut self, x: usize) {
        self.access(x);
        self.splay(x);
        let left = self.nodes[x].left;
        if left != -1 {
            self.nodes[left as usize].parent = -1;
            self.nodes[x].left = -1;
        }
    }

    // Assign value to u and all its ancestors
    pub fn assign_ancestors(&mut self, u: usize, value: T) {
        self.access(u); // expose path root -> x
        self.nodes[u].apply_assign(value); // assign to x and its left subtree (the path)
    }

    // Get value of u
    pub fn get_value(&mut self, u: usize) -> T {
        self.access(u);
        self.splay(u);
        self.nodes[u].val
    }

    // ---------- Core LCT primitives ----------

    fn is_root(&self, x: usize) -> bool {
        let n = &self.nodes[x];
        n.parent == -1 || {
            let p = &self.nodes[n.parent as usize];
            p.left != x as isize && p.right != x as isize
        }
    }

    fn push_down(&mut self, x: usize) {
        if self.nodes[x].rev {
            let node = &mut self.nodes[x];
            swap(&mut node.left, &mut node.right);
            let left = node.left;
            let right = node.right;
            if left != -1 {
                self.nodes[left as usize].rev ^= true
            }
            if right != -1 {
                self.nodes[right as usize].rev ^= true
            }
            self.nodes[x].rev = false;
        }

        if self.nodes[x].has_assign {
            let left = self.nodes[x].left;
            let right = self.nodes[x].right;
            let val = self.nodes[x].val;
            if left != -1 {
                self.nodes[left as usize].apply_assign(val)
            }
            if right != -1 {
                self.nodes[right as usize].apply_assign(val)
            }
            self.nodes[x].has_assign = false;
        }
    }

    fn rotate(&mut self, x: usize) {
        let p = self.nodes[x].parent;
        let g = self.nodes[p as usize].parent;
        self.push_down(p as usize);
        self.push_down(x);

        let is_left = x == self.nodes[p as usize].left as usize;
        let b = if is_left {
            self.nodes[x].right
        } else {
            self.nodes[x].left
        };

        if !self.is_root(p as usize) {
            if p == self.nodes[g as usize].left {
                self.nodes[g as usize].left = x as isize
            } else if p == self.nodes[g as usize].right {
                self.nodes[g as usize].right = x as isize
            }
        }
        self.nodes[x].parent = g;

        if is_left {
            self.nodes[x].right = p;
            self.nodes[p as usize].left = b;
        } else {
            self.nodes[x].left = p;
            self.nodes[p as usize].right = b;
        }

        if b != -1 {
            self.nodes[b as usize].parent = p;
        }
        self.nodes[p as usize].parent = x as isize
    }

    fn splay(&mut self, x: usize) {
        {
            let mut stack = Vec::new();
            let mut y = x;
            stack.push(y);
            while !self.is_root(y) {
                y = self.nodes[y].parent as usize;
                stack.push(y);
            }
            stack.iter().rev().for_each(|&v| self.push_down(v));
        }

        while !self.is_root(x) {
            let p = self.nodes[x].parent as usize;
            let g = self.nodes[p].parent;
            if !self.is_root(p) {
                let up_left = x == self.nodes[p].left as usize;
                let pg_left = p == self.nodes[g as usize].left as usize;
                self.rotate(if up_left == pg_left { p } else { x });
            }
            self.rotate(x);
        }
    }

    // Expose path root -> x; x becomes root of its auxiliary tree
    fn access(&mut self, x: usize) {
        let mut last = -1;
        let mut y = x as isize;
        while y != -1 {
            self.splay(y as usize);
            self.nodes[y as usize].right = last;
            last = y;
            y = self.nodes[y as usize].parent;
        }
        self.splay(x);
    }

    // Make x the root of its represented tree
    fn make_root(&mut self, x: usize) {
        self.access(x);
        self.nodes[x].rev ^= true;
    }
}

#[derive(Clone)]
struct State {
    len: usize,                 // length of longest string in this state
    link: isize,                // suffix link
    next: [isize; Move::COUNT], // transitions

    endp: usize, // one past largest end position
}

impl State {
    fn new() -> Self {
        Self {
            len: 0,
            link: -1,
            next: [-1; Move::COUNT],
            endp: 0,
        }
    }
}

pub struct SuffixAutomaton {
    st: Vec<State>,
    last: usize,

    lct: Option<LinkCutTree<usize>>,

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
            lct: None,
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
            s.endp = pos;
            s
        });

        self.lct.as_mut().map(|l| l.add_vertex(pos));

        let mut p = self.last as isize;
        while p != -1 && self.st[p as usize].next[c as usize] == -1 {
            self.st[p as usize].next[c as usize] = cur as isize;
            p = self.st[p as usize].link;
        }

        if p == -1 {
            self.st[cur].link = 0;
            self.lct.as_mut().map(|l| l.link_parent(cur, 0));
            self.index_of_next = 0; // 0 when not found
        } else {
            let q = self.st[p as usize].next[c as usize];
            let qu = q as usize;

            // save the position AFTER the earlier occurrence
            self.index_of_next = self
                .lct
                .as_mut()
                .map_or(self.st[qu].endp, |l| l.get_value(qu));

            if self.st[p as usize].len + 1 == self.st[qu].len {
                self.st[cur].link = q;
                self.lct.as_mut().map(|l| l.link_parent(cur, qu));
            } else {
                let clone = self.st.len();
                self.st.push(self.st[qu].clone());
                self.st[clone].len = self.st[p as usize].len + 1;

                if let Some(lct) = self.lct.as_mut() {
                    lct.add_vertex(self.index_of_next);
                    lct.cut_parent(clone);
                    lct.link_parent(clone, self.st[qu].link as usize);
                    lct.cut_parent(qu);
                    lct.link_parent(qu, clone);
                    lct.link_parent(cur, clone);
                }

                while p != -1 && self.st[p as usize].next[c as usize] == q {
                    self.st[p as usize].next[c as usize] = clone as isize;
                    p = self.st[p as usize].link;
                }

                self.st[qu].link = clone as isize;
                self.st[cur].link = clone as isize;
            }
        }

        self.last = cur;

        // Propagate the new position upward along suffix links
        if let Some(lct) = self.lct.as_mut() {
            lct.assign_ancestors(self.st[cur].link as usize, pos)
        } else {
            p = self.st[cur].link;
            let mut i: usize = 0;
            while p != -1 {
                self.st[p as usize].endp = pos;
                p = self.st[p as usize].link;
                i += 1;
            }

            // switch to LCT when i >= 8 * (floor(log2(N)) + 1)
            let s = i >> 3;
            if s >= usize::BITS as usize || self.items.len() >> s == 0 {
                self.lct = Some({
                    let mut lct = LinkCutTree::new();
                    self.st.iter().for_each(|s| lct.add_vertex(s.endp));
                    for i in 1..self.st.len() {
                        lct.link_parent(i, self.st[i].link as usize);
                    }
                    lct
                })
            }
        }
    }

    pub fn predict(&self) -> Move {
        self.items[self.index_of_next]
    }
}
