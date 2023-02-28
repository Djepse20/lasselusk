use std::borrow::Borrow;
use std::cell::Ref;
use std::collections::HashMap;
use std::default;
use std::io::Read;
use std::ops::DerefMut;
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
    fn hash_req<H: std::hash::Hasher>(&self , state : &mut H) {
        
        let node = self.borrow();
        if let Some(node) =&node.left {
            let node_val = node.as_ref().borrow().val;
            node_val.hash(state);
             unsafe { Self::hash_req(node.as_ptr().as_ref().unwrap(),state); }

        }
        if let Some(node) =&node.right {
            let node_val = node.as_ref().borrow().val;
            node_val.hash(state);
            unsafe { Self::hash_req(node.as_ptr().as_ref().unwrap(),state); }
        }

    }
}
impl Hash for TreeNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash_req(state);
    }

}




fn check_tree(node : Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut sub_tree : HashMap<TreeNode,Option<Rc<RefCell<TreeNode>>>> = HashMap::new();
    let mut duplicate_sub_tree : HashMap<TreeNode,Option<Rc<RefCell<TreeNode>>>> = HashMap::new();
    check_tree_req(&node, &mut sub_tree,&mut duplicate_sub_tree);
    duplicate_sub_tree.values().map(|val|val.to_owned()).collect()
}


fn check_tree_req(node :& Option<Rc<RefCell<TreeNode>>>, sub_tree : &mut HashMap<TreeNode,Option<Rc<RefCell<TreeNode>>>>, duplicate_sub_tree :  &mut  HashMap<TreeNode,Option<Rc<RefCell<TreeNode>>>>)  {

    if let Some(unwraped_node) = node {
        
        let inner_node = unwraped_node.as_ref().borrow();
        let left_node = inner_node.left.clone();
        let right_node =inner_node.right.clone();
        
        check_tree_req(&left_node ,sub_tree,duplicate_sub_tree);        

        

        check_tree_req(&right_node ,sub_tree,duplicate_sub_tree);    
        match sub_tree.get(&inner_node) {
            Some(tree) => {
                duplicate_sub_tree.insert( TreeNode { val: inner_node.val, left: inner_node.left.clone(), right: inner_node.right.clone() } ,tree.to_owned());
            }
            None => {
                sub_tree.insert(TreeNode { val: inner_node.val, left: inner_node.left.clone(), right: inner_node.right.clone() },  node.to_owned());
            }
        }
    
        

    }

    
}

fn main() {
    let mut node = Some(Rc::new(RefCell::new(TreeNode {val : 1,
        left : Some(Rc::new(RefCell::new(TreeNode { val: 2, 
            left: Some(Rc::new(RefCell::new(TreeNode { val: 4, 
                left: None, 
                right: None 
            }))),
            right: None 
        }))),
        right : Some(Rc::new(RefCell::new(TreeNode { val: 3, 
            left: Some(Rc::new(RefCell::new(TreeNode { val: 3, 
                left: Some(Rc::new(RefCell::new(TreeNode { val: 2, 
                    left: Some(Rc::new(RefCell::new(TreeNode { val: 4, 
                        left: None, 
                        right: None 
                    }))),
                    right: None 
                }))), 
                right: Some(Rc::new(RefCell::new(TreeNode { val: 4, 
                    left: None, 
                    right: None 
                }))) 
            }))), 
            right: None 
        })))

    })));

    println!("{:?}",check_tree(node));
}
