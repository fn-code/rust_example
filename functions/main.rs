fn main() {
    let x = 3;
    let y = sum(x);
    println!("y value is {}", y);

    let mut z:i32 = 1;
    change(&mut z);
    println!("z value is {}", z);

    let res = multiply(8,8);
    println!("the result is {}", res);
}

fn sum(x:i32) -> i32 {
    return x+1
}

fn change(z:&mut i32) {
    *z = 3;
}
fn multiply(num1:i32, num2:i32) -> i32 {
    num1 * num2
}