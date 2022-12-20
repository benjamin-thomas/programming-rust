use std::{env, process};
use std::fs::File;
use std::str::FromStr;
use image::ColorType;
use image::png::PNGEncoder;

use num::Complex;

// Approaches infinity if x > 1 or x < 1
#[allow(dead_code)]
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
#[allow(dead_code)]
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
//noinspection SpellCheckingInspection
#[allow(dead_code)]
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
#[allow(dead_code)]
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

#[allow(dead_code)]
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
#[allow(dead_code)]
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

#[allow(dead_code)]
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        None => None,
        Some((re, im)) => Some(Complex { re, im }),
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625,
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// Given the row and column of a pixel in the output image, return the corresponding point in the
/// complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels. `pixel` is a
/// (column, row) pair indicating a particular pixel in that image.
///
/// The `upper_left` and `lower_right` parameters ar points on the complex plane designating the
/// area our image covers.
#[allow(dead_code)]
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + (pixel.0 as f64) * width / bounds.0 as f64,
        im: upper_left.im - (pixel.1 as f64) * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        Complex {
            re: -0.5,
            im: -0.75
        },
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
    )
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`, which holds one
/// grayscale pixel per byte. The `upper_left` and `lower_right` arguments specify points on the
/// complex plane corresponding to the upper-left and lower-right corners of the pixel buffer.
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = match escape_time(point, 255) {
                None => 0,
                Some(cnt) => 255 - cnt as u8,
            }
        }
    }
}

fn write_image(filename: &str, pixels: &[u8], dim: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    // encoder.write_image(pixels, dim.0 as u32, dim.1 as u32, ColorType::Gray(8)).expect("Could not write image!");
    encoder.encode(pixels, dim.0 as u32, dim.1 as u32, ColorType::Gray(8)).expect("Could not write image");

    Ok(())
}

fn main() {
    /*
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
     */

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPER_LEFT LOWER_RIGHT", args[0]);

        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]);
        process::exit(1);
    }

    let dim = parse_pair(&args[2], 'x').expect("Invalid dimensions");
    let ul = parse_complex(&args[3]).expect("Invalid upper left corner point");
    let lr = parse_complex(&args[4]).expect("Invalid lower right corner point");


    let mut pixels = vec![0; dim.0 * dim.1];
    render(&mut pixels, dim, ul, lr);

    write_image(&args[1], &pixels, dim).expect("Error writing PNG file");


}
