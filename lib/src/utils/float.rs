#![allow(dead_code)]

/// Checks if f32 has remainder
///
/// ## Examples
/// ```ignore
/// use lib::utils::float::has_fract;
///
/// let value = 1.5;
/// assert_eq!(has_fract(value), true);
///
/// let clean_value = 1.0;
/// assert_eq!(has_fract(clean_value), false);
/// ```
pub fn has_fract(value: f32) -> bool {
    let bits = value.to_bits();
    let exponent = (bits >> 23) & 0xff;
    let mantissa = bits & 0x7fffff;
    exponent < 127 || mantissa != 0
}

/// Direct zero check is unreliable
///
/// So we try as best as we can
#[inline]
pub fn is_zero(value: f32) -> bool {
    value.abs() < f32::EPSILON
}

/// Use it whenever you want to feel yourself little more cool 😎
fn q_rsqrt(number: f32) -> f32 {
    let threehalfs = 1.5f32;
    let x2 = number * 0.5f32;
    let mut y = number;

    #[allow(unnecessary_transmutes)]
    let i: i32 = unsafe { std::mem::transmute(y) }; // Evil floating point bit level hacking

    let i = 0x5f3759df - (i >> 1); // What the fuck?

    y = unsafe {
        #[allow(unnecessary_transmutes)]
        std::mem::transmute(i)
    };

    y = y * (threehalfs - (x2 * y * y)); // 1st iteration
    y = y * (threehalfs - (x2 * y * y)); // 2nd iteration (optional)

    y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_rsqrt_still_rocks() {
        let test_subjects = [1.0, 2.0, 4.0, 10.0, 0.25, 100.0];

        for &f in &test_subjects {
            let rust_result = q_rsqrt(f);
            let std_result = 1.0 / f.sqrt();
            let error = (rust_result - std_result).abs();

            assert!(
                error < 0.001,
                "Value: {}, Q_rsqrt: {}, Std: {}, Error: {}",
                f,
                rust_result,
                std_result,
                error
            );
        }
    }

    #[test]
    fn q_rsqrt_should_treat_negatives_in_bad_way() {
        assert!(q_rsqrt(-1.0).is_infinite());
        assert!(q_rsqrt(-4.0).is_infinite());
    }
}
