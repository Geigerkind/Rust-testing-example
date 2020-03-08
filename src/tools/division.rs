#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor.is_nan() {
        return Err("Divisor is NaN".to_string());
    } else if divisor.is_infinite() {
        return Err("Divisor is infinite".to_string());
    } else if divisor == 0.0 {
        return Err("Divisor is 0".to_string());
    }
    Ok(dividend / divisor)
}