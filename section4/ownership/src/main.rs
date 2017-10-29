fn main() {
    foo();
    move_semantics();
    type_copy();
}


fn foo() {
  println!("ownership");
  let v = vec![1, 2, 3];
  println!("v[0] is: {}",v[0])
}

fn move_semantics() {
  println!("move semantics");
  let v = vec![1, 2, 3];

  println!("v[0] is: {}", v[0]);

  let v2 = v;
  // NG
  // println!("v[0] is: {}", v[0]);
  println!("v2[0] is: {}", v2[0]);
}

fn type_copy(){
  println!("copy type");

  fn double(x: i32) -> i32 {
      x * 2
  }
  let a = 5;
  let _x = double(a);
  println!("{}", a);
  fn change_truth(x: bool) -> bool {
    !x
  } 
  let b = true;

  let _y = change_truth(b);
  println!("{}", b);
  
  fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
      // v1とv2についての作業を行う

      // 所有権と関数の結果を返す
      (v1, v2, 42)
  }

  let v1 = vec![1, 2, 3];
  let v2 = vec![1, 2, 3];

  let (v1, v2, answer) = foo(v1, v2);
}
