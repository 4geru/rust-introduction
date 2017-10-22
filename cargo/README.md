## cargoを使って、hello worldを作成した。

### rustのみのファイルのコンパイル

```
$ rustc main.rs
$ ./main
Hello, world!
```

### Cargoのビルドと実行

```
$ cargo build
   Compiling hello_world v0.0.1 (file:///home/yourname/projects/hello_world)
$ ./target/debug/hello_world
Hello, world!
```

```
$ cargo run
     Running `target/debug/hello_world`
Hello, world!
```

### Cargoを自動生成

```
$ cargo new hello_world --bin
```



[https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/getting-started.html](https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/getting-started.html)