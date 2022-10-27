## Description
Benchmarking different ways to pass variables into function in rust.
aka: move, move and return, clone and ref.

## How to run benchmarks
You will need the nightly rust version before starting:
`rustup install nightly`

then you can run tests using `rustup run nightly cargo bench`

## The tests
Trying to move, clone or pass by ref a struct with a container having 10000 elements:

```rust
impl MyStruct {
    pub fn new() -> Self {
        MyStruct {
            some_filed: 15,
            a_container: vec![String::from("hello"); 10000],
        }
    }
}
```

## Results

```
running 4 tests
test tests::bench_clone           ... bench:     972,937 ns/iter (+/- 176,223)
test tests::bench_move            ... bench:     668,986 ns/iter (+/- 156,461)
test tests::bench_move_and_return ... bench:     664,395 ns/iter (+/- 41,668)
test tests::bench_ref             ... bench:     661,754 ns/iter (+/- 22,576)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out; finished in 3.09s
```

clone is clearly the most expensive, and ref is the least expensive.