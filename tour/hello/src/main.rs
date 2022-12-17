fn main() {
    println!("Hello, world!");
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

// cargo watch -x test

#[test]
#[rustfmt::skip]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    /* Prime numbers up to 100:
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97
     */


    assert_eq!(gcd(2 * 3 *  5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
                   3 * 11); // The prime numbers appearing in `n` and `m`
}
