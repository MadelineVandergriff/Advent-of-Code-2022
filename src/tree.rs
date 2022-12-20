#[derive(PartialEq)]
pub struct Tree<T: PartialEq> {
    pub node: T,
    pub children: Vec<Tree<T>>,
}

impl<T: PartialEq> Tree<T> {
    pub fn iter(&self) -> TreeIter<'_, T> {
        TreeIter::from(self)
    }

    pub fn parent<'a, 'b>(&'a self, root: &'b Tree<T>) -> Option<&'b Tree<T>> {
        root.iter()
            .find_map(|x| {
                x.children.iter().find(|x| *x == self)
            })
    }
}

struct TreeIterStackFrame<'a, T: PartialEq> {
    tree: &'a Tree<T>,
    child_idx: usize,
}

pub struct TreeIter<'a, T: PartialEq> {
    trees: Vec<TreeIterStackFrame<'a, T>>,
    prev: Option<&'a Tree<T>>
}

impl<'a, T: PartialEq> Iterator for TreeIter<'a, T> {
    type Item = &'a Tree<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.prev;
        self.prev = self.advance_stack()
            .and_then(|x| Some(x.tree));
        ret
    }
}

impl<'a, T: PartialEq> From<&'a Tree<T>> for TreeIter<'a, T> {
    fn from(value: &'a Tree<T>) -> Self {
        Self {
            trees: vec![TreeIterStackFrame{ tree: value, child_idx: 0 }],
            prev: Some(&value),
        }
    }
}

impl<'a, T: PartialEq> TreeIter<'a, T> {
    fn advance_stack(&mut self) -> Option<&TreeIterStackFrame<'a, T>>{
        let mut last = match self.trees.pop() {
            None => { return None }
            Some(x) => { x }
        };

        match last.tree.children.get(last.child_idx) {
            None => {
                self.advance_stack()
            },
            Some(x) => {
                last.child_idx += 1;
                self.trees.push(last);
                self.trees.push(TreeIterStackFrame{ tree: x, child_idx: 0});
                self.trees.last()
            }
        }
    }
}