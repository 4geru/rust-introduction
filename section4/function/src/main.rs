fn main() {
    let mut x: i32 = 21;
    print_number(x);

    x = add_one(x);
    println!("x is {}", x);

    // without type inference
    let g: fn(i32) -> i32 = plus_one;

    // with type inference
    // let f = plus_one;
    let six = g(5);
    println!("six is {}", six);
}

fn print_number(x: i32){
  println!("x is: {}", x);
}

// NG
// fn print_sum(x , y){
//   println!("sum is: {}", x + y);
// }

fn add_one(x: i32) -> i32 {
  // return value don't have ;
  x + 1
}

fn plus_one(x: i32) -> i32 {
  x + 1
}
