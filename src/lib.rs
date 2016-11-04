pub struct BinaryTree<'a> {
    value: i32,
    left: Option<&'a BinaryTree<'a>>,
    right: Option<&'a BinaryTree<'a>>,
}

impl<'a> BinaryTree<'a> {
    fn new(v: i32, l: Option<&'a BinaryTree<'a>>, r: Option<&'a BinaryTree<'a>>) -> BinaryTree<'a> {
        BinaryTree {
            value: v,
            left: l,
            right: r,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let l2 = BinaryTree::new(3, None, None);
        let l1 = BinaryTree::new(1, Some(&l2), None);
        let r1 = BinaryTree::new(2, None, None);
        let t = BinaryTree::new(2, Some(&l1), Some(&r1));
    }
}
