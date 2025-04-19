#![feature(test)]

extern crate test;
use test::Bencher;

#[macro_use]
extern crate string_concat;
#[macro_use(concat_strs)]
extern crate concat_strs;
#[macro_use(concat_string)]
extern crate concat_string;
extern crate joinery;
use joinery::prelude::*;

static HOST: &str = "127.0.0.1:";
static PORT: &str = "8080";
static HOST_PORT: &str = "127.0.0.1:8080";

////
#[bench]
fn array_concat(b: &mut Bencher) {
    b.iter(|| {
        let datetime: &str = &[HOST, PORT].concat();
        test::black_box(datetime);
    });
}

#[test]
fn array_concat_test() {
    let datetime: &str = &[HOST, PORT].concat();
    assert_eq!(String::from(HOST_PORT), datetime);
}

////
#[bench]
fn array_join(b: &mut Bencher) {
    b.iter(|| {
        let datetime: &str = &[HOST, PORT].join(PORT);
        test::black_box(datetime);
    });
}

#[test]
fn array_join_test() {
    let datetime: &str = &[HOST, PORT].join(PORT);
    assert_eq!(String::from(HOST_PORT), datetime);
}

////
#[bench]
fn array_join_long(b: &mut Bencher) {
    b.iter(|| {
        let datetime: &str = &[HOST, PORT].join("");
        test::black_box(datetime);
    });
}

#[test]
fn array_join_long_test() {
    let datetime: &str = &[HOST, PORT].join("");
    assert_eq!(String::from(HOST_PORT), datetime);
}

////
#[bench]
fn collect_from_array_to_string(b: &mut Bencher) {
    let list = [HOST, PORT];
    b.iter(|| {
        let datetime: String = list.iter().map(|x| *x).collect();
        test::black_box(datetime);
    });
}

#[test]
fn collect_from_array_to_string_test() {
    let list = [HOST, PORT];
    let datetime: String = list.iter().map(|x| *x).collect();
    assert_eq!(String::from(HOST_PORT), datetime);
}

////
#[bench]
fn collect_from_vec_to_string(b: &mut Bencher) {
    let list = vec![HOST, PORT];
    b.iter(|| {
        let datetime: String = list.iter().map(|x| *x).collect();
        test::black_box(datetime);
    });
}

#[test]
fn collect_from_vec_to_string_test() {
    let list = vec![HOST, PORT];
    let datetime: String = list.iter().map(|x| *x).collect();
    assert_eq!(String::from(HOST_PORT), datetime);
}

////
#[bench]
fn format_macro(b: &mut Bencher) {
    b.iter(|| {
        let datetime: &str = &format!("{}{}", HOST, PORT);
        test::black_box(datetime);
    });
}

#[test]
fn format_macro_test() {
    let datetime: &str = &format!("{}{}", HOST, PORT);
    assert_eq!(String::from(HOST_PORT), datetime);
}

/// Implicit named arguments were added in Rust 1.58
#[bench]
fn format_macro_implicit_args(b: &mut Bencher) {
    b.iter(|| {
        let datetime: &str = &format!("{HOST}{PORT}");
        test::black_box(datetime);
    });
}

#[test]
fn format_macro_implicit_args_test() {
    let datetime: &str = &format!("{HOST}{PORT}");
    assert_eq!(String::from(HOST_PORT), datetime);
}

////
// #[bench]
// #[cfg(unix)]
// fn from_bytes(b: &mut Bencher) {
//     use std::ffi::OsStr;
//     use std::os::unix::ffi::OsStrExt;
//     use std::slice;

//     b.iter(|| {
//         let bytes = unsafe { slice::from_raw_parts(HOST.as_ptr(), 20) };

//         let datetime = OsStr::from_bytes(bytes);
//         test::black_box(datetime);
//     });
// }

// #[test]
// #[cfg(unix)]
// fn from_bytes_test() {
//     use std::ffi::OsStr;
//     use std::os::unix::ffi::OsStrExt;
//     use std::slice;

//     let bytes = unsafe { slice::from_raw_parts(HOST.as_ptr(), 20) };

//     let datetime = OsStr::from_bytes(bytes);

//     assert_eq!(
//         String::from(DATETIME),
//         datetime.to_owned().into_string().unwrap()
//     );
// }

////
#[bench]
fn mut_string_push_str(b: &mut Bencher) {
    b.iter(|| {
        let mut datetime = String::new();
        datetime.push_str(HOST);
        datetime.push_str(PORT);
        test::black_box(datetime);
    });
}

#[test]
fn mut_string_push_str_test() {
    let mut datetime = String::new();
    datetime.push_str(HOST);
    datetime.push_str(PORT);
    assert_eq!(String::from(HOST_PORT), datetime);
}

////
#[bench]
fn mut_string_push_string(b: &mut Bencher) {
    b.iter(|| {
        let mut datetime = Vec::<String>::new();
        datetime.push(String::from(HOST));
        datetime.push(String::from(PORT));
        let datetime = datetime.join("");
        test::black_box(datetime);
    });
}

#[test]
fn mut_string_push_string_test() {
    let mut datetime = Vec::<String>::new();
    datetime.push(String::from(HOST));
    datetime.push(String::from(PORT));
    let datetime = datetime.join("");
    assert_eq!(String::from(HOST_PORT), datetime);
}

////
#[bench]
fn mut_string_with_capacity_push_str(b: &mut Bencher) {
    b.iter(|| {
        let mut datetime = String::with_capacity(20);
        datetime.push_str(HOST);
        datetime.push_str(PORT);
        test::black_box(datetime);
    });
}

#[test]
fn mut_string_with_capacity_push_str_test() {
    let mut datetime = String::with_capacity(20);
    datetime.push_str(HOST);
    datetime.push_str(PORT);
    assert_eq!(String::from(HOST_PORT), datetime);
}

////
#[bench]
fn mut_string_with_capacity_push_str_char(b: &mut Bencher) {
    b.iter(|| {
        let mut datetime = String::with_capacity(20);
        datetime.push_str(HOST);
        datetime.push_str(PORT);
        test::black_box(datetime);
    });
}

#[test]
fn mut_string_with_capacity_push_str_char_test() {
    let mut datetime = String::with_capacity(20);
    datetime.push_str(HOST);
    datetime.push_str(PORT);
    assert_eq!(String::from(HOST_PORT), datetime);
}

////
#[bench]
fn mut_string_with_too_little_capacity_push_str(b: &mut Bencher) {
    b.iter(|| {
        let mut datetime = String::with_capacity(2);
        datetime.push_str(HOST);
        datetime.push_str(PORT);
        test::black_box(datetime);
    });
}

#[test]
fn mut_string_with_too_little_capacity_push_str_test() {
    let mut datetime = String::with_capacity(2);
    datetime.push_str(HOST);
    datetime.push_str(PORT);
    assert_eq!(String::from(HOST_PORT), datetime);
}

////
#[bench]
fn mut_string_with_too_much_capacity_push_str(b: &mut Bencher) {
    b.iter(|| {
        let mut datetime = String::with_capacity(200);
        datetime.push_str(HOST);
        datetime.push_str(PORT);
        test::black_box(datetime);
    });
}

#[test]
fn mut_string_with_too_much_capacity_push_str_test() {
    let mut datetime = String::with_capacity(200);
    datetime.push_str(HOST);
    datetime.push_str(PORT);
    assert_eq!(String::from(HOST_PORT), datetime);
}

////
#[bench]
fn string_from_all(b: &mut Bencher) {
    b.iter(|| {
        let datetime: &str = &(String::from(HOST) + &String::from(PORT));
        test::black_box(datetime);
    });
}

#[test]
fn string_from_all_test() {
    let datetime: &str = &(String::from(HOST) + &String::from(PORT));
    assert_eq!(String::from(HOST_PORT), datetime);
}

////
#[bench]
fn string_from_plus_op(b: &mut Bencher) {
    b.iter(|| {
        let datetime: &str = &(String::from(HOST) + PORT);
        test::black_box(datetime);
    });
}

#[test]
fn string_from_plus_op_test() {
    let datetime: &str = &(String::from(HOST) + PORT);
    assert_eq!(String::from(HOST_PORT), datetime);
}

////
#[bench]
fn to_owned_plus_op(b: &mut Bencher) {
    b.iter(|| {
        let datetime: &str = &(HOST.to_owned() + PORT);
        test::black_box(datetime);
    });
}

#[test]
fn to_owned_plus_op_test() {
    let datetime: &str = &(HOST.to_owned() + PORT);
    assert_eq!(String::from(HOST_PORT), datetime);
}

////
#[bench]
fn to_string_plus_op(b: &mut Bencher) {
    b.iter(|| {
        let datetime: &str = &(HOST.to_string() + PORT);
        test::black_box(datetime);
    });
}

#[test]
fn to_string_plus_op_test() {
    let datetime: &str = &(HOST.to_string() + PORT);
    assert_eq!(String::from(HOST_PORT), datetime);
}

// ===== MACRO TESTS =====

/// https://crates.io/crates/concat-in-place
#[bench]
fn concat_in_place_macro(b: &mut Bencher) {
    b.iter(|| {
        let mut url = String::new();
        let datetime = concat_in_place::strcat!(&mut url, HOST PORT);
        test::black_box(datetime);
    });
}

#[test]
fn concat_in_place_macro_test() {
    let mut url = String::new();
    let datetime = concat_in_place::strcat!(&mut url, HOST PORT);
    assert_eq!(&String::from(HOST_PORT), datetime);
}

/// https://crates.io/crates/string_concat
#[bench]
fn string_concat_macro(b: &mut Bencher) {
    b.iter(|| {
        let datetime = &string_concat::string_concat!(HOST, PORT);
        test::black_box(datetime);
    });
}

#[test]
fn string_concat_macro_test() {
    let datetime = &string_concat::string_concat!(HOST, PORT);
    assert_eq!(&String::from(HOST_PORT), datetime);
}

/// https://crates.io/crates/concat_strs
/// This macro breaks RustAnalyzer (https://github.com/rust-analyzer/rust-analyzer/issues/6835)
#[bench]
fn concat_strs_macro(b: &mut Bencher) {
    b.iter(|| {
        let datetime = &concat_strs!(HOST, PORT);
        test::black_box(datetime);
    });
}

#[test]
fn concat_strs_macro_test() {
    let datetime = &concat_strs!(HOST, PORT);
    assert_eq!(&String::from(HOST_PORT), datetime);
}

/// https://crates.io/crates/concat-string
#[bench]
fn concat_string_macro(b: &mut Bencher) {
    b.iter(|| {
        let datetime = concat_string!(HOST, PORT);
        test::black_box(datetime);
    });
}

#[test]
fn concat_string_macro_test() {
    let datetime = &concat_string!(HOST, PORT);
    assert_eq!(&String::from(HOST_PORT), datetime);
}

/// https://crates.io/crates/joinery
#[bench]
fn joinery(b: &mut Bencher) {
    let vec = vec![HOST, PORT];
    b.iter(|| {
        let datetime = &vec.iter().join_concat().to_string();
        test::black_box(datetime);
    });
}

#[test]
fn joinery_test() {
    let vec = vec![HOST, PORT];
    let datetime = &vec.iter().join_concat().to_string();
    assert_eq!(&String::from(HOST_PORT), datetime);
}
