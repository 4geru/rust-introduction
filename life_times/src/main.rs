fn main() {
  foo();
  example();
}

fn foo(){
  struct Foo<'a> {
    x: &'a i32,
  }

  let y = &5;
  let f = Foo {x: y};
  println!("x is {}", f.x);
}

fn example(){
  // fn print(s: &str); // 省略された形
  // fn print<'a>(s: &'a str); // 展開した形

  // fn debug(lvl: u32, s: &str); // 省略された形
  // fn debug<'a>(lvl: u32, s: &'a str); // 展開された形

  // // 前述の例では`lvl`はライフタイムを必要としません。なぜなら、それは参照（`&`）
  // // ではないからです。（参照を含む`struct`のような）参照に関係するものだけがライ
  // // フタイムを必要とします。

  // fn substr(s: &str, until: u32) -> &str; // 省略された形
  // fn substr<'a>(s: &'a str, until: u32) -> &'a str; // 展開された形

  // fn get_str() -> &str; // 不正。入力がない

  // fn frob(s: &str, t: &str) -> &str; // 不正。入力が2つある
  // fn frob<'a, 'b>(s: &'a str, t: &'b str) -> &str; // 展開された形。出力ライフタイムが決まらない

  // fn get_mut(&mut self) -> &mut T; // 省略された形
  // fn get_mut<'a>(&'a mut self) -> &'a mut T; // 展開された形

  // fn args<T:ToCStr>(&mut self, args: &[T]) -> &mut Command; // 省略された形
  // fn args<'a, 'b, T:ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; // 展開された形

  // fn new(buf: &mut [u8]) -> BufWriter; // 省略された形
  // fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>; // 展開された形
}