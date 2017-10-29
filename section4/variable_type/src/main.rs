fn main() {
    boolean();
    char();
    number();
    array();
    slice();
    tuple();
    function();
}

fn boolean(){
    let x = true;
    let y:bool = false;
    println!(">> boolean type");
    println!("the value of x is {}, and value of y is {}", x, y);
}

fn char(){
    let x = 'x';
    let two_hearts = '💕';
    println!(">> char type");
    println!("char examples are {}, {}", x, two_hearts)
}

fn number(){
    let x: i32 = 123;
    let y: u32 = 123;
    let z: f32 = 3.14;
    println!(">> number type");
    println!("the value of x is {}",x);
    println!("the value of y is {}",y);
    println!("the value of z is {}",z);
}

fn array(){
    let a = [1, 2, 3]; // a: [i32; 3]
    let mut m = [1, 2, 3]; // m: [i32; 3]
    let f = m[1];
    m = a;
    // set default value
    let b = [f; 20];
    println!(">> array type");
    println!("m first number {}, array length is {}", m[0], m.len());
    println!("b first number {}, array length is {}", b[0], b.len());
}

fn slice(){
    println!(">> slice type");
    let a = [0, 1, 2, 3, 4];
    let complete = &a[..];
    let middle = &a[1..4];    
    println!("slice a -> complete");
    for x in 0..a.len() {
        println!("{} -> {}", a[x], complete[x]);
    }
    println!("slice index -> middle");
    for x in 0..middle.len() {
        println!("{} -> {}", x, middle[x]);
    }
}

fn tuple(){
    println!(">> fuple type");
    let x = (1, "hello");
    let y: (i32, &str) = (1, "hello");
    println!("the value of x is {}, the value of is {}", x.0, y.0);
    let mut a = (1, 2);
    println!("tuple value of a are {} and {}", a.0, a.1);
    let b = (2, 3);
    a = b;
    let (q, r) = a;
    println!("value of q is {} and value of r is {}", q, r);
}

fn function(){
    println!(">> function type");
    fn foo(x:i32) -> i32 { x }
    let x: fn(i32) -> i32 = foo;
    println!("{}", x(12));
}
