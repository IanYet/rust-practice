#![allow(unused_variables, dead_code, unused_assignments)]
use std::ops::{Range, RangeInclusive};

// 移除某个部分让代码工作
fn q01() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;

    let z: i32 = 10; // 这里 z 的类型是?
}

// 填空
fn q02() {
    let v: u16 = 38_u8 as u16;
    println!("{}", v)
}

// 修改 `assert_eq!` 让代码工作
fn q03() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));
}

// 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// 填空，让代码工作
fn q04() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
}

// 解决代码中的错误和 `panic`
fn q05() {
    let v1 = 251_u32 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("{},{}", v1, v2);
}

// 修改 `assert!` 让代码工作
fn q06() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);
}

// 将 ? 替换成你的答案
fn q07() {
    let x = 1_000.000_1; // f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64
}

fn q08() {
    assert!(0.1_f32 + 0.2 == 0.3);
}
fn q09() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c as u8);
    }
}

// 填空
fn q10() {
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
}

// 填空，并解决错误
fn q11() {
    // 整数加法
    assert!(1u32 + 2 == 3);

    // 整数减法
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9.6_f32 / 3.2 == 3.0); // error ! 修改它让代码工作

    assert!(24 % 5 == 4);

    // 逻辑与或非操作
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // 位操作
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

// 修改一行让代码正常打印
fn q12() {
    let c1 = '中';
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}

// 让代码工作：修改 `assert!` 中的 `4`
use std::mem::size_of_val;
fn q13() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!")
}

// 使用两种方法让代码工作起来
fn q14() {
    let v = {
        let mut x = 1;
        x += 2
    };

    assert_eq!(v, ());
}

fn q15() {
    // 不要修改下面两行代码!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    q15()
}
