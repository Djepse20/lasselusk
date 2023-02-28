use std::borrow::Borrow;
use std::cell::Ref;
use std::{rc::Rc, cell::RefCell};
use std::hash::Hash;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
impl TreeNode {
    fn hash_req<H: std::hash::Hasher>(node :Rc<RefCell<TreeNode>> , state : &mut H) {
        
        let node : Ref<TreeNode> = node.as_ref().borrow();
        if let Some(node) =&node.left {
            let node_val = node.as_ref().borrow().val;
            node_val.hash(state);
            Self::hash_req(node.to_owned(),state);
        }
        if let Some(node) =&node.right {
            let node_val = node.as_ref().borrow().val;
            node_val.hash(state);
            Self::hash_req(node.to_owned(),state);
        }

    }
}
impl Hash for TreeNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let node = TreeNode {val : self.val, left : self.left.clone(), right :self.left.clone()};
        Self::hash_req(Rc::new(RefCell::new(node)), state);
    }

}


fn main() {

}
