fn main() {
  borrowing();
  mut_borrowing();
  iteraltor();
}

fn borrowing(){
  fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
      // v1とv2についての作業を行う
      let v3:i32 = v1[0] + v2[0];
      // 答えを返す
      42 + v3
  }

  let v1 = vec![1, 2, 3];
  let v2 = vec![1, 2, 3];

  let answer = foo(&v1, &v2);
  println!("{}",answer);
}

fn mut_borrowing(){
  let mut x = 5;
  {
      let y = &mut x;
      *y += 1;
  }
  println!("{}", x);
}

fn iteraltor(){
  let mut v = vec![1, 2, 3];

  for i in &v {
      println!("{}", i);
  }
}