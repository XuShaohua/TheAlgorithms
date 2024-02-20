pub fn get_prime_list(max_num: usize) -> Vec<usize> {
    let mut list = vec![true; max_num + 1];
    let sqrt = (max_num as f64).sqrt() as usize;
    let mut mul;
    for i in 2..(sqrt + 2) {
        for j in 2..max_num {
            mul = i * j;
            if mul > max_num {
                break;
            }
            list[mul] = false;
        }
    }

    let mut result = vec![];
    for i in 2..max_num {
        if list[i] {
            result.push(i);
        }
    }

    result
}

pub trait IsPrime {
    fn is_prime(&self) -> bool;
}

macro_rules! is_prime_impl {
    ( $( $t:ident )* ) => {
        $(
            impl IsPrime for $t {
                fn is_prime(&self) -> bool {
                    if self <= &0 {
                        return false;
                    }
                    if self % 2 == 0 {
                        return false;
                    }

                    let mut d = 3;
                    while &(d * d) <= self {
                        if self % d == 0 {
                            return false;
                        }
                        d += 2;
                    }
                    return true;
                }
            }
        )*
    };
}

is_prime_impl!(u8 i8 u16 i16 u32 i32 usize isize u64 i64 u128 i128);

#[derive(Debug, Clone, Copy)]
pub struct PrimeFactor {
    pub num: usize,
    pub count: u16,
}

pub fn get_prime_factors(num: usize, primes: &[usize]) -> Vec<PrimeFactor> {
    let mut result = Vec::<PrimeFactor>::new();
    let mut rem = num;
    let root = (num as f64).sqrt().ceil() as usize;
    for p in primes {
        if rem == 1 {
            break;
        }
        if p > &root {
            if rem > 1 {
                result.push(PrimeFactor { num: rem, count: 1 });
            }
            break;
        }
        let mut prime_factor = PrimeFactor { num: *p, count: 0 };
        while rem != 1 {
            if rem % *p == 0 {
                prime_factor.count += 1;
                rem /= *p;
            } else {
                break;
            }
        }
        if prime_factor.count > 0 {
            result.push(prime_factor);
        }
    }

    result
}

pub trait GetFactors {
    type Output;
    fn get_factors(self) -> Self::Output;
    fn get_factors_cache(self, v: &mut Self::Output);
}

macro_rules! get_factors_impl {
    ( $( $t:ident )* ) => {
        $(
            impl GetFactors for $t {
                type Output = Vec<$t>;
                fn get_factors(self) -> Self::Output {
                    let mut factors = Vec::new();
                    if self <= 0 {
                        return factors;
                    }
                    for i in 1..self {
                        if self % i == 0 {
                            factors.push(i);
                        }
                    }
                    return factors;
                }

                fn get_factors_cache(self, v: &mut Self::Output) {
                    v.clear();
                    if self <= 0 {
                        return;
                    }
                    for i in 1..self {
                        if self % i == 0 {
                            v.push(i);
                        }
                    }
                }
            }
        )*
    };
}
get_factors_impl!(u8 i8 u16 i16 u32 i32 usize isize u64 i64 u128 i128);

pub fn get_prime_factor_num(num: usize, primes: &[usize]) -> u32 {
    let mut count = 0;
    let mut rem = num;
    let root = (num as f64).sqrt().ceil() as usize;
    for p in primes {
        if rem == 1 {
            break;
        }
        if p > &root {
            if rem > 1 {
                count += 1;
            }
            break;
        }
        let mut is_factor = false;
        while rem != 1 {
            if rem % *p == 0 {
                rem /= *p;
                is_factor = true;
            } else {
                break;
            }
        }
        if is_factor {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_factors() {
        let factors = 12_u32.get_factors();
        assert_eq!(factors, vec![1, 2, 3, 4, 6]);
    }

    #[test]
    fn test_is_prime() {
        assert!(31_u32.is_prime());
        assert!(!64_u16.is_prime());
    }
}
