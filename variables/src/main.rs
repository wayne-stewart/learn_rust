fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let mut x = 3;
    x = x + 1;
    println!("The value of x is: {}", x);

    let x : u32 = 0xFFFFFFFF;
    println!("The value of x is: {}", x);

    let x : u64 = 0xFFFFFFFFFFFFFFFF;
    println!("The value of x is: {}", x);

    let x = (1,2,3,4);
    println!("The value of x is: {}", x.0);

    let mut x = 0;
    while x < 5 {
        x += 1;
        println!("The value of x is: {}", x);
    }

    for a in 1..5 {
        println!("The value of a is: {}", a);
    }
}
