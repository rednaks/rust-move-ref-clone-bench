#![feature(test)]

extern crate test;

#[derive(Debug, Clone)]
pub struct MyStruct {
    some_filed: i32,
    a_container: Vec<String>,
}

impl MyStruct {
    pub fn new() -> Self {
        MyStruct {
            some_filed: 15,
            a_container: vec![String::from("hello"); 10000],
        }
    }
}

pub fn do_stuff_move(st: MyStruct) {
    let _num = st.some_filed.clone();
    let _c = st.a_container.clone();
}

pub fn do_stuff_move_and_return(st: MyStruct) -> MyStruct {
    let _num = st.some_filed.clone();
    let _c = st.a_container.clone();
    st
}

pub fn do_stuff_ref(st: &MyStruct) {
    let _num = st.some_filed.clone();
    let _c = st.a_container.clone();
}

pub fn my_test_move() {
    let st = MyStruct::new();
    do_stuff_move(st)
}

pub fn my_test_clone() {
    let st = MyStruct::new();
    do_stuff_move(st.clone())
}

pub fn my_test_move_and_return() {
    let st = MyStruct::new();
    let _st = do_stuff_move(st);
}

pub fn my_test_ref() {
    let st = MyStruct::new();
    do_stuff_ref(&st)
}

fn build_test_data(size: usize) -> Vec<i32> {
    let mut num_list = vec![0; size];
    let mut idx = -1;
    let num_list = num_list
        .iter()
        .map(|&_| {
            idx = idx + 1;
            idx
        })
        .collect();

    num_list
}

pub fn my_imperative_mult() {
    let mut num_list = build_test_data(1000000);
    let mut result = 0;
    for n in num_list {
        if n % 2 == 0 {
            result += n * 10;
        }
    }
}



pub fn my_func_mult() {
    let mut num_list = build_test_data(1000000);
    let result = num_list
        .iter()
        .filter(|&n| n % 2 == 0)
        .map(|&n| n * 10)
        .reduce(|n, m| n + m).unwrap_or(0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::black_box;
    use test::Bencher;

    #[bench]
    fn bench_move(b: &mut Bencher) {
        b.iter(|| black_box(my_test_move()))
    }

    #[bench]
    fn bench_clone(b: &mut Bencher) {
        b.iter(|| black_box(my_test_clone()))
    }
    #[bench]
    fn bench_move_and_return(b: &mut Bencher) {
        b.iter(|| black_box(my_test_move_and_return()))
    }

    #[bench]
    fn bench_ref(b: &mut Bencher) {
        b.iter(|| black_box(my_test_ref()))
    }

    #[bench]
    fn bench_imperative(b: &mut Bencher) {
        b.iter(|| black_box(my_imperative_mult()))
    }

    #[bench]
    fn bench_functional(b: &mut Bencher) {
        b.iter(|| black_box(my_func_mult()))
    }
}
