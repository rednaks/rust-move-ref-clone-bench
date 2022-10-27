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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use test::black_box;

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
}
