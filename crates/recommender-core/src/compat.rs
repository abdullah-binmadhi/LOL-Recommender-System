//! Small compatibility seam for the captured JavaScript scoring contract.
use serde_json::Value;

pub fn truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(value) => *value,
        Value::Number(value) => value.as_f64().is_some_and(|n| n != 0.0 && !n.is_nan()),
        Value::String(value) => !value.is_empty(),
        _ => true,
    }
}

pub fn num(value: &Value, key: &str) -> f64 {
    match value.get(key) {
        Some(Value::Number(n)) => n.as_f64().unwrap_or(f64::NAN),
        Some(Value::Null) | None => 0.0,
        Some(Value::Bool(b)) => {
            if *b {
                1.0
            } else {
                0.0
            }
        }
        Some(Value::String(s)) if s.trim().is_empty() => 0.0,
        Some(Value::String(s)) => s.trim().parse().unwrap_or(f64::NAN),
        _ => f64::NAN,
    }
}

pub fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or("undefined")
}

pub fn js_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => "null".into(),
        Value::Array(values) => values
            .iter()
            .map(|v| {
                if v.is_null() {
                    String::new()
                } else {
                    js_string(v)
                }
            })
            .collect::<Vec<_>>()
            .join(","),
        Value::Object(_) => "[object Object]".into(),
        _ => value.to_string(),
    }
}

/// JS toFixed rounds the exact binary value, with ties away from zero, not ties-even.
/// Only 0 and 3 decimal places are used by the scoring explanations.
pub fn fixed(value: f64, digits: usize) -> String {
    assert!(digits <= 12);
    if !value.is_finite() {
        return value.to_string();
    }
    let absolute = value.abs();
    if absolute >= 1e21 {
        return value.to_string();
    }
    let bits = absolute.to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as i32;
    let mantissa = (bits & ((1_u64 << 52) - 1)) | if exponent == 0 { 0 } else { 1_u64 << 52 };
    let shift = if exponent == 0 {
        -1074
    } else {
        exponent - 1023 - 52
    };
    let scaled = u128::from(mantissa) * 10_u128.pow(digits as u32);
    let rounded = if shift >= 0 {
        scaled << shift
    } else if -shift >= 128 {
        0
    } else {
        let shift = (-shift) as u32;
        let quotient = scaled >> shift;
        let remainder = scaled - (quotient << shift);
        quotient + u128::from(remainder >= (1_u128 << (shift - 1)))
    };
    let sign = if value < 0.0 { "-" } else { "" };
    if digits == 0 {
        return format!("{sign}{rounded}");
    }
    let scale = 10_u128.pow(digits as u32);
    format!(
        "{sign}{}.{:0width$}",
        rounded / scale,
        rounded % scale,
        width = digits
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixed_uses_binary_value_and_js_tie_rule() {
        assert_eq!(fixed(60.5, 0), "61");
        assert_eq!(fixed(-60.5, 0), "-61");
        assert_eq!(fixed(1.005, 2), "1.00");
        assert_eq!(fixed(2.675, 2), "2.67");
        assert_eq!(fixed(0.0, 3), "0.000");
        assert_eq!(fixed(-0.0, 0), "0");
    }
}
