// Test the first list

#[cfg(test)]
mod test_1 {

    use crate::first::List;

    #[test]
    fn test1a() {

        let mut list  = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

    }

}

// Test the second list

#[cfg(test)]
mod test_2 {

    use crate::second::List;

    #[test]
    fn test2a_base() {

        let mut list  = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

    }

    #[test]
    fn test_2b_peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
    }

    #[test]
    fn test_2c_peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn test_2d_into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]    fn test_2e_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn test_2f_iter_mut() {

        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));

        iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));

    }

    #[test]
    fn test_2f_iter_mut_str() {

        let mut list = List::new();
        list.push("A".to_string()); list.push("B".to_string()); list.push("C".to_string());

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut "C".to_string()));
        assert_eq!(iter.next(), Some(&mut "B".to_string()));
        assert_eq!(iter.next(), Some(&mut "A".to_string()));
        assert_eq!(iter.next(), None);

        iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut "C".to_string()));
        assert_eq!(iter.next(), Some(&mut "B".to_string()));
        assert_eq!(iter.next(), Some(&mut "A".to_string()));
        assert_eq!(iter.next(), None);

    }

    #[test]
    fn test_2f_iter_mut_nocopy() {

        #[derive(Debug)]
        #[derive(Eq, PartialEq)]
        struct Testit {
            s : String,
            i : u32,
        }

        let mut list = List::new();

        list.push(Testit{s: "AA".to_string(), i: 1});
        list.push(Testit{s: "BB".to_string(), i: 2});
        list.push(Testit{s: "CC".to_string(), i: 3});

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut Testit{s: "CC".to_string(), i: 3}));
        assert_eq!(iter.next(), Some(&mut Testit{s: "BB".to_string(), i: 2}));
        assert_eq!(iter.next(), Some(&mut Testit{s: "AA".to_string(), i: 1}));
        assert_eq!(iter.next(), None);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut Testit{s: "CC".to_string(), i: 3}));
        assert_eq!(iter.next(), Some(&mut Testit{s: "BB".to_string(), i: 2}));
        assert_eq!(iter.next(), Some(&mut Testit{s: "AA".to_string(), i: 1}));
        assert_eq!(iter.next(), None);

    }

    #[cfg(test)]
    mod test_2 {
        
        use crate::third::List;
    
        #[test]
        fn basics() {
            let list = List::new();
            assert_eq!(list.head(), None);
    
            let list = list.append(1).append(2).append(3);
            assert_eq!(list.head(), Some(&3));
    
            let list = list.tail();
            assert_eq!(list.head(), Some(&2));
    
            let list = list.tail();
            assert_eq!(list.head(), Some(&1));
    
            let list = list.tail();
            assert_eq!(list.head(), None);
    
            // Make sure empty tail works
            let list = list.tail();
            assert_eq!(list.head(), None);
    
        }
    }
    

}
