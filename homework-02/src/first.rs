/// BST struct needed with root element of type Link.
/// BST::new() method needed
/// Link is an enum with 2 instances, Empty and More. More contains a boxed Node
/// Node is a struct containing i32 element and 2 child Links: left and right
/// insert method needed which accepts i32 and returns bol indicating if the insertion was
/// successful
/// search method needed which accepts i32 and return bool - true if the element was found
pub struct BST {
    root: Link
}

impl BST {
    pub fn new() -> Self {
        BST { root: Link::Empty }
    }

    pub fn insert(&self) {
        println!("{}", "testing");
    }
}

enum Link {
    Empty,
    More(Box<Node>)
}

struct Node {
    element: i32,
    left: Link,
    right: Link
}

#[cfg(test)]
mod tests {
    use super::{BST};

    #[test]
    fn testing_bst() {
        let bst = BST::new();
        bst.insert();
    }
}
