// TODO: everything
use std::mem;

struct Node {
    elem: i32,
    next: Link
}

enum Link {
    Empty,
    Next(Box<Node>)
}

pub struct List{
	head: Link
}

impl List{
	pub fn new() -> Self {
		List {head : Link::Empty}
	}

/*
	pub fn push(&mut self, element: i32){
		self.head = Link::Next(Box::new(Node {elem: element, next: self.head}));
	}
*/

	pub fn push(&mut self, elem: i32) {
    	let new_node = Box::new(Node {
    	    elem: elem,
    	    //next: self.head
	        next: mem::replace(&mut self.head, Link::Empty) //mem::replace needed to swap objects prior to changing ownership
    	});

    	self.head = Link::Next(new_node);
	}

	pub fn pop(&mut self) -> Option<i32> {
		//match &self.head {  //need to borrow self because we don't own it by value
		let result;
		match mem::replace(&mut self.head, Link::Empty) {  //need to borrow self because we don't own it by value //need to also do mem replace to get it by value to remove items
			Link::Empty => {
				result = None;
			}
			Link::Next(n) => {
				result = Some(n.elem);
				self.head = n.next;
			}
		};
		result
	}
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::Next(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}