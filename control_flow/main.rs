fn main() {
    let mut x = 0;
    loop {
        println!("x : {}", x);
        if x == 10 {
            break
        }
        x = x +1;
    };

    conditions();
    loop_element();
    loop_number();

}

fn conditions() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);
}

fn loop_element() {
    let a = [2,12,44,2,5,66];

    for el in a.iter() {
        println!("the value is: {}", el);
    }
}

fn loop_number() {
    for i in (1..4).rev() {
        println!("value : {}", i);
    }
}