use linked_lists::second::List;
use std::fmt::Debug;

// Return the type of the entity pointed to by the reference passed

fn get_type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

// Print the list

fn print_list<T: Debug>(l: &mut List<T>) {

    for node in l.iter_mut() {
        println!("{:?} of type {} @ {:p}", node, get_type_of(&node), node);
    }

}

// Sample main program

fn main() {

    #[derive(Debug)]
    #[derive(Eq, PartialEq)]
    struct Testit {
        s : String,
        i : u32,
    }

    let mut list = List::new();

    let mut a = Testit{s: "AA".to_string(), i: 1};
    let b = Testit{s: "BB".to_string(), i: 2};
    let c = Testit{s: "CC".to_string(), i: 3};
    println!("a: {:p}, b: {:p}, c: {:p}", &a, &b, &c);

    a.i = 15;

    list.push(a);
    list.push(b);
    list.push(c);

    print_list(&mut list);

    // Change a value in the list
    let mut iter = list.iter_mut();
    if let Some(c1) = iter.next() {
        println!("c1: {:p}: {}", c1, get_type_of(&c1));
        c1.i = 5;
    }

    print_list(&mut list);

}
