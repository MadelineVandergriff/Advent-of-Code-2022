use std::cell::RefCell;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::rc::{Rc, Weak};

pub enum TreeErrors {
    ParentUpgradeError
}

impl Debug for TreeErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeErrors::ParentUpgradeError => f.write_str("failed to upgrade parent pointer")
        }
    }
}

impl Display for TreeErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (self as &dyn Debug).fmt(f)
    }
}

impl Error for TreeErrors {}

pub struct Tree<T> {
    pub node: T,
    pub parent: Option<Weak<RefCell<Tree<T>>>>,
    pub children: Vec<Rc<RefCell<Tree<T>>>>
}

impl<T> Tree<T> {
    pub fn new(node: T) -> Rc<RefCell<Tree<T>>> {
        Rc::new(RefCell::new(Tree {
            node,
            parent: None,
            children: vec![],
        }))
    }
}

pub trait TreePointer<T> where Self: Sized {
    fn add_child(&self, node: T) -> Result<Self, Box<dyn Error>>;
    fn get_child<F: Fn(&T) -> bool>(&self, predicate: F) -> Option<Self>;
    fn parent(&self) -> Option<Self>;
    fn drop(self) -> Result<(), Box<dyn Error>>;
    fn iter(&self) -> TreeIter<T>;
}

impl<T> TreePointer<T> for Rc<RefCell<Tree<T>>> {
    fn add_child(&self, node: T) -> Result<Rc<RefCell<Tree<T>>>, Box<dyn Error>> {
        let children = &mut self.try_borrow_mut()?.children;
        children.push(Rc::new(RefCell::new(Tree {
            node,
            parent: Some(Rc::downgrade(self)),
            children: vec![],
        })));
        Ok(children.last().unwrap().clone())
    }

    fn get_child<F: Fn(&T) -> bool>(&self, predicate: F) -> Option<Self> {
        self.borrow()
            .children
            .iter()
            .find(|x| predicate(&x.borrow().node))
            .and_then(|x| Some(x.clone()))
    }

    fn parent(&self) -> Option<Self> {
        self.borrow()
            .parent
            .as_ref()
            .and_then(|x| x.upgrade())
    }

    fn drop(self) -> Result<(), Box<dyn Error>> {
        self.borrow()
            .parent
            .clone()
            .and_then(|x| x.upgrade())
            .ok_or_else(|| Box::new(TreeErrors::ParentUpgradeError))?
            .try_borrow_mut()?
            .children
            .retain(|x| !Rc::ptr_eq(x, &self));
        Ok(())
    }

    fn iter(&self) -> TreeIter<T> {
        TreeIter {
            ptr: self.clone(),
            child_idx: vec![-1]
        }
    }
}

pub struct TreeIter<T> {
    ptr: Rc<RefCell<Tree<T>>>,
    child_idx: Vec<isize>
}

impl<T> Iterator for TreeIter<T> {
    type Item = Weak<RefCell<Tree<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.child_idx.last_mut();
        if idx.is_none() { return None; }
        let idx = idx.unwrap();
        *idx += 1;

        if *idx == 0 {
            return Some(Rc::downgrade(&self.ptr))
        }

        let child = self
            .ptr.borrow()
            .children.get(*idx as usize - 1)
            .and_then(|x| Some(x.clone()));

        if child.is_none() {
            self.child_idx.pop();
            let parent = self.ptr.borrow().parent.as_ref()
                .and_then(|x| x.upgrade());
            if parent.is_none() {
                return None;
            }
            self.ptr = parent.unwrap();
            return self.next();
        }

        self.ptr = child.unwrap();
        self.child_idx.push(-1);
        self.next()
    }
}