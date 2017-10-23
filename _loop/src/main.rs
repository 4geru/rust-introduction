fn main() {
    _loop();
    _while();
    _for();
    simple();
    label();
}

fn _loop(){
  let mut i:i32 = 0;
  loop{
    println!("loop forever");
    if i == 2 {
      break;
    }
    i += 1;
  }
}

fn _while(){
  let mut x = 5;
  let mut done = false;
  while !done {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 {
        done = true;
    }
  }
}

fn _for(){
  for x in 0..10 {
    print!( "{} ", x);
  }
  println!("");

  for (i,j) in (5..10).enumerate() {
    println!("i = {} and j = {}", i, j);
  }
}

fn simple(){
  let mut x = 5;

  loop {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 { break; }
  }
  for x in 0..10 {
    if x % 2 == 0 { continue; }

    print!("{} ", x);
  }println!();
}

fn label(){
  'outer: for x in 0..10 {
    'inner: for y in 0..10 {
        if x % 2 == 0 { continue 'outer; } // x のループを継続
        if y % 2 == 0 { continue 'inner; } // y のループを継続
        println!("x: {}, y: {}", x, y);
    }
  }
}