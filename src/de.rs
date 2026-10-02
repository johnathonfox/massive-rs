//! Exact decimal decoding for REST response models.
//!
//! Massive sends prices, sizes and ratios as plain JSON numbers. Decoding them
//! into `f64` and back rounds values that have no exact binary form, so every
//! numeric model field is a [`Decimal`] read from the raw JSON token text via
//! [`RawValue`]. The value never passes through `f64`.
//!
//! These helpers need the `serde_json` deserializer itself (the one reqwest's
//! `Response::json` uses). They do not work through `serde_json::Value`,
//! because a `Value` has already decoded its numbers to `f64`.

use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{de::Error as _, Deserialize, Deserializer};
use serde_json::value::RawValue;

/// Parse one raw JSON scalar: a number token (`182.31`, `1.5e-7`) or a quoted
/// numeric string (`"182.31"`).
pub(crate) fn parse_raw(raw: &str) -> Result<Decimal, String> {
    let text = raw.trim().trim_matches('"');
    if text.contains(['e', 'E']) {
        Decimal::from_scientific(text).map_err(|e| format!("invalid decimal {text:?}: {e}"))
    } else {
        Decimal::from_str_exact(text).map_err(|e| format!("invalid decimal {text:?}: {e}"))
    }
}

/// An optional numeric field. `null` and an absent key (with `default`) both
/// read `None`.
pub(crate) fn decimal_opt<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Decimal>, D::Error> {
    let raw: Option<&RawValue> = Deserialize::deserialize(d)?;
    match raw {
        None => Ok(None),
        Some(r) if r.get().trim() == "null" => Ok(None),
        Some(r) => parse_raw(r.get()).map(Some).map_err(D::Error::custom),
    }
}

/// An optional map of numeric values (e.g. sector exposure weights).
pub(crate) fn decimal_map_opt<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<HashMap<String, Decimal>>, D::Error> {
    let raw: Option<HashMap<String, &RawValue>> = Deserialize::deserialize(d)?;
    match raw {
        None => Ok(None),
        Some(map) => map
            .into_iter()
            .map(|(k, v)| parse_raw(v.get()).map(|d| (k, d)))
            .collect::<Result<_, _>>()
            .map(Some)
            .map_err(D::Error::custom),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[derive(Deserialize)]
    struct Holder {
        #[serde(default, deserialize_with = "decimal_opt")]
        v: Option<Decimal>,
        #[serde(default, deserialize_with = "decimal_map_opt")]
        m: Option<HashMap<String, Decimal>>,
    }

    fn v(json: &str) -> Option<Decimal> {
        serde_json::from_str::<Holder>(json).unwrap().v
    }

    #[test]
    fn number_token_is_exact() {
        // 0.1 + 0.2 style values: f64 cannot hold these exactly.
        assert_eq!(
            v(r#"{"v": 182.31}"#),
            Some(Decimal::from_str("182.31").unwrap())
        );
        assert_eq!(
            v(r#"{"v": 0.30000000000000004}"#).unwrap().to_string(),
            "0.30000000000000004"
        );
    }

    #[test]
    fn scale_is_preserved() {
        assert_eq!(v(r#"{"v": 140.50}"#).unwrap().to_string(), "140.50");
    }

    #[test]
    fn integers_and_quoted_strings_decode() {
        assert_eq!(v(r#"{"v": 123456789}"#), Some(Decimal::from(123_456_789)));
        assert_eq!(
            v(r#"{"v": "12.5"}"#),
            Some(Decimal::from_str("12.5").unwrap())
        );
    }

    #[test]
    fn scientific_notation_decodes() {
        assert_eq!(
            v(r#"{"v": 1.5e-7}"#),
            Some(Decimal::from_str("0.00000015").unwrap())
        );
        assert_eq!(v(r#"{"v": 2.5E+3}"#), Some(Decimal::from(2500)));
    }

    #[test]
    fn null_and_absent_read_none() {
        assert_eq!(v(r#"{"v": null}"#), None);
        assert_eq!(v(r#"{}"#), None);
    }

    #[test]
    fn non_numeric_is_an_error() {
        assert!(serde_json::from_str::<Holder>(r#"{"v": "abc"}"#).is_err());
        assert!(serde_json::from_str::<Holder>(r#"{"v": true}"#).is_err());
    }

    #[test]
    fn map_values_decode_exactly() {
        let h: Holder = serde_json::from_str(r#"{"m": {"Tech": 0.3125, "Energy": 1e-2}}"#).unwrap();
        let m = h.m.unwrap();
        assert_eq!(m["Tech"].to_string(), "0.3125");
        assert_eq!(m["Energy"], Decimal::from_str("0.01").unwrap());
    }

    #[test]
    fn a_real_model_decodes_exactly() {
        let agg: crate::models::aggs::Agg = serde_json::from_str(
            r#"{"o": 125.07, "c": 0.30000000000000004, "v": 1.2345678e8, "t": 1672723200000}"#,
        )
        .unwrap();
        assert_eq!(agg.open.unwrap().to_string(), "125.07");
        assert_eq!(agg.close.unwrap().to_string(), "0.30000000000000004");
        assert_eq!(agg.volume, Some(Decimal::from(123_456_780)));
        assert_eq!(agg.high, None);
        assert_eq!(agg.timestamp, Some(1_672_723_200_000));
    }
}
