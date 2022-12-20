use std::str::FromStr;

use num::Complex;

// Approaches infinity if x > 1 or x < 1
fn square_loop(mut x: f64) {
    let mut idx = 0;
    loop {
        idx += 1;
        x = x * x;
        // eprintln!("[{}] {}", idx, x);
        // std::thread::sleep(std::time::Duration::from_millis(10 as u64));

        if x == f64::INFINITY || x == 0.0 {
            if x == f64::INFINITY {
                println!("Approaches infinity at iteration: {}", idx);
            }
            if x == 0.0 {
                println!("Approaches 0 at iteration: {}", idx);
            }
            return;
        }
    }
}

// Approaches infinity if c > 0.25 or < -2.0
fn sq_add_loop(c: f64) {
    let mut x = 0.0;
    let mut idx = 0;
    loop {
        idx += 1;
        x = x * x + c;
        eprintln!("[{}] {}", idx, x);
        if idx >= 10 {
            eprintln!("WIP: early exit");
            return;
        }
        // std::thread::sleep(std::time::Duration::from_millis(1000 as u64));
    }
}

/*
    The Mandelbrot set is defined as the set of complex numbers `c` for which `z`
    does not fly out to infinity.
*/
fn sq_cmplx_lp(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let mut idx = 0;
    loop {
        idx += 1;
        z = z * z + c;
        eprintln!("[{}] {}", idx, z);
        if idx >= 10 {
            eprintln!("WIP: early exit");
            return;
        }
    }
}

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius 2 centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn print_escape(c: Complex<f64>) {
    println!("({}, {}): {:?}", c.re, c.im, escape_time(c, 2000));
}

/// Parse the string `s` as a coordinate pair, lika `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument and <left> and <right> are
/// both strings that can be parsed by `T::from_str`. `separator` must be an
/// ASCII character.
///
/// If `s` has the proper form, return `Some<x, y>`. If it doesn't parse
/// correctly, return `None`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20x", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
       None => None,
        Some((re, im)) => Some(Complex { re, im})
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}

fn main() {
    square_loop(0.999999999999999); // closest representable value before 1. I can represent one more digit if 0.
    square_loop(1.000000000000001); // closest representable value after 1.
    // sq_add_loop(0.250000000100000);

    println!("---");
    sq_add_loop(-1.99000000000000);
    println!("---");
    sq_add_loop(-1.99999999999999);
    sq_cmplx_lp(Complex { re: 0.1, im: 0.2 });

    print_escape(Complex { re: 0.1, im: 0.2 });
    print_escape(Complex { re: 0.1, im: 0.3 });
    print_escape(Complex { re: 0.1, im: 1.1 });
    print_escape(Complex { re: 1.1, im: 1.1 });
    print_escape(Complex { re: 0.1, im: 1.4 });
    print_escape(Complex { re: 0.1, im: 2.0 });
}
