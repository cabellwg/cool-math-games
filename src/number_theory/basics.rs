/// Calculates the greatest common denominator of two integers.
///
/// Uses a division-based implementation of the Euclidean algorithm.
///
/// # Arguments
///
/// * `m`: A signed integer
/// * `n`: A second signed integer
///
/// # Examples
///
/// ```
/// let m = 45;
/// let n = 21;
/// let answer = gcd(m, n);
///
/// assert_eq!(3, answer);
/// ```
pub fn gcd(m: i128, n: i128) -> i128 {
    if m == 0 || n == 0 { return 0 }

    let mut m = m.abs();
    let mut n = n.abs();

    while n != 0 {
        let tmp = n;
        n = m % n;
        m = tmp;
    }

    m
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_normal_case() {
        assert_eq!(3, gcd(45, 21));
    }

    #[test]
    fn test_gcd_reverse_order() {
        assert_eq!(15, gcd(15, 45));
    }

    #[test]
    fn test_gcd_negative() {
        assert_eq!(4, gcd(-16, 12));
        assert_eq!(4, gcd(16, -12));
        assert_eq!(4, gcd(-16, -12));
    }

    #[test]
    fn test_gcd_zero() {
        assert_eq!(0, gcd(0, 5));
        assert_eq!(0, gcd(5, 0));
        assert_eq!(0, gcd(0, 0));
    }
}
