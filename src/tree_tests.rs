// According to the rust book doc.rust-lang.org/book/ 
//  unit tests should be placed in the same folder as the
//  modules they are testing, integration tests are placed /tests/

#[cfg(test)]
mod tree_tests {
    use crate::tree::Tree;

    #[test]
    fn test_new() {
        let root = Tree::<i32>::new();
        match root {
            Tree::Node { value: _, left: _, right: _ } => assert!(false),
            Tree::Empty => assert!(true),
        }
    }
    
    #[test]
    fn test_create() {
        let root = Tree::<i32>::create(10);
        match root {
            Tree::Node { value, left: _, right: _ } => assert_eq!(value, 10),
            Tree::Empty => assert!(false),
        }
    }

    #[test]
    fn test_insert_left() {
        let mut root = Tree::<i32>::create(30);
        root.insert(15);
        match root {
            Tree::Node { value: _, left, right: _ } => match *left {
                Tree::Node { value, left: _, right: _ } => assert_eq!(value, 15),
                Tree::Empty => assert!(false),
            },
            Tree::Empty => assert!(false),
        }
    }
    #[test]
    fn test_insert_right() {
        let mut root = Tree::<i32>::create(20);
        root.insert(25);
        match root {
            Tree::Node { value: _, left: _, right } => match *right {
                Tree::Node { value, left: _, right: _ } => assert_eq!(value, 25),
                Tree::Empty => assert!(false),
            },
            Tree::Empty => assert!(false),
        }
    }
    #[test]
    fn test_insert_equal() {
        let mut root = Tree::create(14);
        root.insert(14);
        match root {
            Tree::Node { value, left, right } => {
                assert_eq!(value, 14);
                match *left {
                    Tree::Node { value: _, left: _, right: _ } => assert!(false),
                    Tree::Empty => assert!(true),
                };
                match *right {
                    Tree::Node { value: _, left: _, right: _ } => assert!(false),
                    Tree::Empty => assert!(true),
                }
            },
            Tree::Empty => assert!(false),
        }
    }

    #[test]
    fn test_find() {
        let mut root = Tree::<i32>::create(250);
        root.insert(200);
        root.insert(300);
        root.insert(100);
        root.insert(225);
        root.insert(275);
        root.insert(275);
        root.insert(350);
        root.insert(50);
        root.insert(175);
        root.insert(270);
        root.insert(190);
        // find root value
        assert!(root.find(250));
        // find right side of tree 
        assert!(root.find(275));
        //find deep in left side of tree
        assert!(root.find(190));
    }

    #[test]
    fn test_lca_unsigned() {
        let mut root = Tree::<u32>::create(250);
        root.insert(200);
        root.insert(300);
        root.insert(100);
        root.insert(225);
        root.insert(275);
        root.insert(275);
        root.insert(350);
        root.insert(50);
        root.insert(175);
        root.insert(270);
        root.insert(190);
        // test lca from left
        assert_eq!(root.lca(50, 190), 100);
        // flip the values on the left, should be same result as above
        assert_eq!(root.lca(190, 50), 100);
        // test lca from right
        assert_eq!(root.lca(270, 350), 300);
        // flip values on right, should be same as above
        assert_eq!(root.lca(350, 270), 300);
        // test with vales on right and left of root respectively
        assert_eq!(root.lca(190, 270), 250);
        // flip above values
        assert_eq!(root.lca(270, 190), 250);
    }

    #[test]
    fn test_lca_signed() {
        let mut root = Tree::create(30);
        root.insert(-10);
        root.insert(50);
        root.insert(-180);
        root.insert(12);
        root.insert(35);
        root.insert(57);
        root.insert(-200);
        root.insert(-70);
        root.insert(13);
        root.insert(52);
        root.insert(-50);
        // test left side of tree
        assert_eq!(root.lca(-70, 12), -10);
        // flip above values
        assert_eq!(root.lca(12, -70), -10);
        // test right side of tree
        assert_eq!(root.lca(52, 35), 50);
        // flip above values
        assert_eq!(root.lca(35, 52), 50);
        // test values on either side of tree
        assert_eq!(root.lca(-70, 57), 30);
        // flip above values
        assert_eq!(root.lca(57, -70), 30);
    }

    #[test]
    fn test_lca_float() {
        let mut root = Tree::create(10.5);
        root.insert(6.8);
        root.insert(12.7);
        root.insert(-2.1);
        root.insert(7.2);
        root.insert(11.2);
        root.insert(368.9);
        root.insert(-16.7);
        root.insert(-1.0);
        // test left side of tree
        assert_eq!(root.lca(-16.7, -1.0), -2.1);
        // flip above values
        assert_eq!(root.lca(-1.0, -16.7), -2.1);
        // test right side of tree
        assert_eq!(root.lca(11.2, 12.7), 12.7);
        // flip above values
        assert_eq!(root.lca(12.7, 11.2), 12.7);
        // test values on either side of tree
        assert_eq!(root.lca(-1.0, 368.9), 10.5);
        // flip above values
        assert_eq!(root.lca(368.9, -1.0), 10.5);
    }
    #[test]
    #[should_panic]
    // Due to the potential issues from the PartialOrd trait, 
    //  invalid inputs such as NaN should be tested for
    fn test_insert_illegal_values() {
        let mut root = Tree::<f64>::new();
        root.insert(f64::NAN);
    }
    #[test]
    #[should_panic]
    fn test_find_illegal_values() {
        let mut root = Tree::<f64>::create(100.0);
        root.find(f64::NAN);
    }
    #[test]
    #[should_panic]
    fn test_lca_empty() {
        let mut root = Tree::<u32>::new();
        root.lca(33, 12);
    }
    #[test]
    #[should_panic]
    fn test_lca_value_not_in_tree() {
        let mut root = Tree::create(10.5);
        root.insert(6.8);
        root.insert(12.7);
        root.insert(-2.1);
        root.insert(7.2);
        root.insert(11.2);
        root.insert(368.9);
        root.insert(-16.7);
        root.insert(-1.0);
        println!("{}", root.lca(7.2, 13.122));
    }

}
