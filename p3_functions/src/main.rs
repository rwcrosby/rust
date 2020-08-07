fn main() {

    let rslt = expression(45);
    println!("The result is {}", rslt);

    println!("Some Loop {}", someloop(5));

    forloop(4);

}

fn expression(x: i32) -> i32 {
    if x > 10 {
        10 * x
    } else {
        23 * x
    }
}

fn someloop(rep: u32) -> u32 {

    let mut i = 0;

    loop {

        if i >= rep {
            break i;
        }

        i += 1;
    }

}

fn forloop(rep: u32) {

    for i in 1..rep {
        println!("i {}", i)
    }

}
