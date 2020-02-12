
fn main() {

    let s = String::from("rust example");
    let s1 = s;
    // if we use s we got error value borrowed here, because s1 is borrowed s
    // println!("{}", s)
    // solution use println!("{}", s)
    println!("{}", s1);

    // another solution if we want to use variable s, we use clone function
    let a = String::from("hola example");
    let _a1 = a.clone();
    // if we don't want to use a1 we can add underscore like _a1
    println!("a : {}, a1 : {}", a, a);

    // stack-only data:copy
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);


    // ownership and function
    let sf = String::from("rust ownership");
    takes_ownership(sf);

    let xf = 5;
    makes_copy(xf);

    // return value scope
    let b = give_ownership();
    let b2 = String::from("holla");
    let b3 = takes_and_gives_back(b2.clone());

    println!("{} {} {}", b,b2,b3);

}

fn takes_ownership(str: String) {
    println!("{}", str);
}

fn makes_copy(str: i32) {
    println!("{}", str);
}

fn give_ownership() -> String {
    let str = String::from("hello");
    str
}

fn takes_and_gives_back(str: String) -> String {
    str
}