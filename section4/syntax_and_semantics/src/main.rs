fn main() {
    println!("Hello, world!");
  // variable
    // NG
    // let x = 5;
    // x = 10;

    // OK
    let mut x = 5;
    println!("The value of x is: {}",x);
    x = 10;
    println!("The value of x is: {}",x);

  // scope
    {
      let y: i32 = 3;
      println!("the value of x is {} and value of y is {}", x, y);
    }

    x = 8;
    {
      println!("{}", x);
      let x = 12;
      println!("{}", x);
    }
    println!("{}", x);
    x = 42;
    println!("{}", x);
}
