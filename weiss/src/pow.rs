#[must_use]
#[inline]
const fn is_even(n: usize) -> bool {
    n % 2 == 0
}

fn pow(x: i32, n: usize) -> i32 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return x;
    }
    if is_even(n) {
        pow(x * x, n / 2)
    } else {
        pow(x * x, n / 2) * x
    }
}

fn main() {
    let r = pow(3, 4);
    println!("r: {r}");
}
