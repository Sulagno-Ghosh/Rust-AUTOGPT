
fn add_two_integers(a: i32, b: i32) -> i32 {
    a+b
}

fn subtract(a:i32, b: i32) -> i32{
    a-b
}

fn multiply(a: i32, b: i32) -> i32{
    a/b
}

fn divide(a: i32, b:i32) -> i32{
    a/b
}

fn main() {
    
    let result = add_two_integers(5, 7);
    let result1: i32 = subtract(5, 7);
    let result2: i32 = multiply(7, 5);
    let result3: i32 = divide(8, 4);
    println!("The result is {}",result);
    println!("The result is {}",result1);
    println!("The result is {}",result2);
    println!("The result is {}",result3);

}
