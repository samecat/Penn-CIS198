// TODO: everything
//use std::mem;

#[derive(Debug)]  //enables debug printing
struct Node {
    elem: i32,
    left: Link,
    right: Link
}

#[derive(Debug)]
enum Link {
    Empty,
    Target(Box<Node>)
}

impl Link{

	fn recursive_search(&self, i: i32) -> bool{
		match self {
			Link::Empty => false,
			Link::Target(n)	=> {
				if i == n.elem{
					true
				}
				else if i > n.elem {
					n.right.recursive_search(i)
				}
				else {
					n.left.recursive_search(i)
				}
			}
		}
	}

	fn recursive_insert(&mut self, i: i32) -> bool{
		match self{
			Link::Empty => {
				*self = Link::Target(Box::new(Node{elem: i, right: Link::Empty, left: Link::Empty}));
				true
			}
			Link::Target(n)	=> {
				if i == n.elem{
					false
				}
				else if i > n.elem {
					n.right.recursive_insert(i)
				}
				else {
					n.left.recursive_insert(i)
				}
			}		
		}
	}	

}


#[derive(Debug)]
pub struct BST{
	head: Link
}

impl BST {
	pub fn new() -> Self{
		BST {head : Link::Empty}
	}

	pub fn insert(&mut self, i: i32) -> bool{
		self.head.recursive_insert(i)
	}

	pub fn search(&self, i: i32) -> bool{
		self.head.recursive_search(i)
	}

}

#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn basics() {
    	let mut a_bst = BST::new();

    	//Check empty tree behaves right
    	assert_eq!(a_bst.search(11), false);

        // Populate Tree
        a_bst.insert(5);
        a_bst.insert(11);
        a_bst.insert(15);

        // Check search
        assert_eq!(a_bst.search(5), true);
        assert_eq!(a_bst.search(11), true);
        assert_eq!(a_bst.search(15), true);
        assert_eq!(a_bst.search(10), false);

        // Insert some more just to make sure nothing's corrupted
        a_bst.insert(9);
        a_bst.insert(10);

        assert_eq!(a_bst.search(5), true);
        assert_eq!(a_bst.search(11), true);
        assert_eq!(a_bst.search(15), true);
        assert_eq!(a_bst.search(9), true);
        assert_eq!(a_bst.search(10), true);
        assert_eq!(a_bst.search(0), false);

    }
}