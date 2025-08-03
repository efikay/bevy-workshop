/// Checks if f32 has remainder
///
/// ## Examples
/// ```
/// let value = 1.5;
/// assert_eq!(has_fractional_part(value), true);
///
/// let clean_value = 1.0;
/// assert_eq!(has_fractional_part(clean_value), false);
/// ```
#[inline]
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
