// Euclid's algorithm
fn gcd(mut m: u64, mut n: u64) -> u64 {
    if m == 0 || n == 0 {
        return 0;
    }
    if m < n {
        return gcd(n, m);
    }

    let mut rem;
    while n > 0 {
        rem = m % n;
        m = n;
        n = rem;
    }
    m
}

fn main() {
    let r = gcd(1989, 1590);
    println!("r: {r}");
}
