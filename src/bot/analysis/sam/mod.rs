use std::mem::swap;

use strum::EnumCount;

use crate::bot::Move;

struct Node<T> {
    left: isize,
    right: isize,
    parent: isize,
    rev: bool,

    has_assign: bool,
    assign_val: T,

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
            assign_val: T::default(),

            val,
        }
    }

    fn apply_assign(&mut self, val: T) {
        self.val = val;
        self.assign_val = val;
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

    // Change parent of u to newParent
    pub fn change_parent(&mut self, u: usize, new_parent: usize) {
        self.cut_parent(u); // detach from old parent if any
        self.link_parent(u, new_parent); // attach under newParent
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

    fn is_root(&self, u: usize) -> bool {
        let x = &self.nodes[u];
        x.parent == -1
            || (self.nodes[x.parent as usize].left != u as isize
                && self.nodes[x.parent as usize].right != u as isize)
    }

    fn push_down(&mut self, u: usize) {
        if self.nodes[u].rev {
            let node = &mut self.nodes[u];
            swap(&mut node.left, &mut node.right);
            let left = node.left;
            let right = node.right;
            if left != -1 {
                self.nodes[left as usize].rev ^= true
            }
            if right != -1 {
                self.nodes[right as usize].rev ^= true
            }
            self.nodes[u].rev = false;
        }

        if self.nodes[u].has_assign {
            let left = self.nodes[u].left;
            let right = self.nodes[u].right;
            let val = self.nodes[u].assign_val;
            if left != -1 {
                self.nodes[left as usize].apply_assign(val)
            }
            if right != -1 {
                self.nodes[right as usize].apply_assign(val)
            }
            self.nodes[u].has_assign = false;
        }
    }

    fn rotate(&mut self, u: usize) {
        let p = self.nodes[u].parent;
        let g = self.nodes[p as usize].parent;
        self.push_down(p as usize);
        self.push_down(u);

        let is_left = u == self.nodes[p as usize].left as usize;
        let b = if is_left {
            self.nodes[u].right
        } else {
            self.nodes[u].left
        };

        if !self.is_root(p as usize) {
            if p == self.nodes[g as usize].left {
                self.nodes[g as usize].left = u as isize
            } else if p == self.nodes[g as usize].right {
                self.nodes[g as usize].right = u as isize
            }
        }
        self.nodes[u].parent = g;

        if is_left {
            self.nodes[u].right = p;
            self.nodes[p as usize].left = b;
        } else {
            self.nodes[u].left = p;
            self.nodes[p as usize].right = b;
        }

        if b != -1 {
            self.nodes[b as usize].parent = p;
        }
        self.nodes[p as usize].parent = u as isize
    }

    fn splay(&mut self, u: usize) {
        {
            let mut stack = Vec::new();
            let mut y = u;
            stack.push(y);
            while !self.is_root(y) {
                y = self.nodes[y].parent as usize;
                stack.push(y);
            }
            stack.iter().rev().for_each(|&v| self.push_down(v));
        }

        while !self.is_root(u) {
            let p = self.nodes[u].parent as usize;
            let g = self.nodes[p].parent;
            if !self.is_root(p) {
                let up_left = u == self.nodes[p].left as usize;
                let pg_left = p == self.nodes[g as usize].left as usize;
                self.rotate(if up_left == pg_left { p } else { u });
            }
            self.rotate(u);
        }
    }

    // Expose path root -> x; x becomes root of its auxiliary tree
    fn access(&mut self, u: usize) {
        let mut last = -1;
        let mut y = u as isize;
        while y != -1 {
            self.splay(y as usize);
            self.nodes[y as usize].right = last;
            last = y;
            y = self.nodes[y as usize].parent;
        }
        self.splay(u);
    }

    // Make x the root of its represented tree
    fn make_root(&mut self, x: usize) {
        self.access(x);
        self.nodes[x].rev ^= true;
    }

    // Link x as a child of p (assumes no cycle is created)
    fn link_parent(&mut self, x: usize, p: usize) {
        self.make_root(x);
        self.nodes[x].parent = p as isize;
    }

    // Cut x from its parent, if any
    fn cut_parent(&mut self, u: usize) {
        self.access(u);
        self.splay(u);
        let left = self.nodes[u].left;
        if left != -1 {
            self.nodes[left as usize].parent = -1;
            self.nodes[u].left = -1;
        }
    }
}

#[derive(Clone)]
struct State {
    len: usize,                 // length of longest string in this state
    link: isize,                // suffix link
    next: [isize; Move::COUNT], // transitions
}

impl State {
    fn new() -> Self {
        Self {
            len: 0,
            link: -1,
            next: [-1; Move::COUNT],
        }
    }
}

pub struct SuffixAutomaton {
    st: Vec<State>,
    last: usize,

    lct: LinkCutTree<usize>,

    items: Vec<Move>,
    index_of_next: usize,
}

impl SuffixAutomaton {
    pub fn new() -> Self {
        let mut st = Vec::new();
        st.push(State::new()); // root
        let mut lct = LinkCutTree::new();
        lct.add_vertex(0);
        Self {
            st,
            last: 0,
            lct,
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
            s
        });

        self.lct.add_vertex(pos);

        let mut p = self.last as isize;
        while p != -1 && self.st[p as usize].next[c as usize] == -1 {
            self.st[p as usize].next[c as usize] = cur as isize;
            p = self.st[p as usize].link;
        }

        if p == -1 {
            self.st[cur].link = 0;
            self.lct.change_parent(cur, 0);
            self.index_of_next = 0; // 0 when not found
        } else {
            let q = self.st[p as usize].next[c as usize];
            let qu = q as usize;

            // save the position AFTER the earlier occurrence
            self.index_of_next = self.lct.get_value(qu);

            if self.st[p as usize].len + 1 == self.st[qu].len {
                self.st[cur].link = q;
                self.lct.change_parent(cur, qu);
            } else {
                let clone = self.st.len();
                self.st.push(self.st[qu].clone());
                self.st[clone].len = self.st[p as usize].len + 1;

                self.lct.add_vertex(self.index_of_next);
                self.lct.change_parent(clone, self.st[qu].link as usize);
                self.lct.change_parent(qu, clone);
                self.lct.change_parent(cur, clone);

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
        self.lct.assign_ancestors(self.st[cur].link as usize, pos);
    }

    pub fn predict(&self) -> Move {
        self.items[self.index_of_next]
    }
}
