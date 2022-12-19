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
    }
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
}
