use super::Value;
use crate::{OwnedValue, ValueTrait};
use float_cmp::approx_eq;

impl<'a> PartialEq for Value<'a> {
    fn eq(&self, other: &Self) -> bool {
        #[allow(clippy::default_trait_access)]
        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::Bool(v1), Self::Bool(v2)) => v1.eq(v2),
            (Self::I64(v1), Self::I64(v2)) => v1.eq(v2),
            (Self::F64(v1), Self::F64(v2)) => approx_eq!(f64, *v1, *v2),
            (Self::String(v1), Self::String(v2)) => v1.eq(v2),
            (Self::Array(v1), Self::Array(v2)) => v1.eq(v2),
            (Self::Object(v1), Self::Object(v2)) => v1.eq(v2),
            _ => false,
        }
    }
}

impl<'v, T> PartialEq<&T> for Value<'v>
where
    Value<'v>: PartialEq<T>,
{
    fn eq(&self, other: &&T) -> bool {
        self == *other
    }
}

impl<'a> PartialEq<OwnedValue> for Value<'a> {
    fn eq(&self, other: &OwnedValue) -> bool {
        // We only need to implement this once
        other.eq(self)
    }
}

impl<'v> PartialEq<()> for Value<'v> {
    fn eq(&self, _other: &()) -> bool {
        self.is_null()
    }
}

impl<'v> PartialEq<bool> for Value<'v> {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool().map(|t| t == *other).unwrap_or_default()
    }
}

impl<'v> PartialEq<str> for Value<'v> {
    fn eq(&self, other: &str) -> bool {
        self.as_str().map(|t| t == other).unwrap_or_default()
    }
}

impl<'v> PartialEq<&str> for Value<'v> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str().map(|t| t == *other).unwrap_or_default()
    }
}

impl<'v> PartialEq<String> for Value<'v> {
    fn eq(&self, other: &String) -> bool {
        self.as_str().map(|t| t == other).unwrap_or_default()
    }
}

impl<'v> PartialEq<i8> for Value<'v> {
    fn eq(&self, other: &i8) -> bool {
        self.as_i8().map(|t| t == *other).unwrap_or_default()
    }
}

impl<'v> PartialEq<i16> for Value<'v> {
    fn eq(&self, other: &i16) -> bool {
        self.as_i16().map(|t| t == *other).unwrap_or_default()
    }
}

impl<'v> PartialEq<i32> for Value<'v> {
    fn eq(&self, other: &i32) -> bool {
        self.as_i32().map(|t| t == *other).unwrap_or_default()
    }
}

impl<'v> PartialEq<i64> for Value<'v> {
    fn eq(&self, other: &i64) -> bool {
        self.as_i64().map(|t| t == *other).unwrap_or_default()
    }
}

impl<'v> PartialEq<i128> for Value<'v> {
    fn eq(&self, other: &i128) -> bool {
        self.as_i128().map(|t| t == *other).unwrap_or_default()
    }
}

impl<'v> PartialEq<u8> for Value<'v> {
    fn eq(&self, other: &u8) -> bool {
        self.as_u8().map(|t| t == *other).unwrap_or_default()
    }
}

impl<'v> PartialEq<u16> for Value<'v> {
    fn eq(&self, other: &u16) -> bool {
        self.as_u16().map(|t| t == *other).unwrap_or_default()
    }
}

impl<'v> PartialEq<u32> for Value<'v> {
    fn eq(&self, other: &u32) -> bool {
        self.as_u32().map(|t| t == *other).unwrap_or_default()
    }
}

impl<'v> PartialEq<u64> for Value<'v> {
    fn eq(&self, other: &u64) -> bool {
        self.as_u64().map(|t| t == *other).unwrap_or_default()
    }
}

impl<'v> PartialEq<usize> for Value<'v> {
    fn eq(&self, other: &usize) -> bool {
        self.as_usize().map(|t| t == *other).unwrap_or_default()
    }
}

impl<'v> PartialEq<u128> for Value<'v> {
    fn eq(&self, other: &u128) -> bool {
        self.as_u128().map(|t| t == *other).unwrap_or_default()
    }
}

impl<'v> PartialEq<f32> for Value<'v> {
    fn eq(&self, other: &f32) -> bool {
        self.as_f32().map(|t| t == *other).unwrap_or_default()
    }
}
impl<'v> PartialEq<f64> for Value<'v> {
    fn eq(&self, other: &f64) -> bool {
        self.as_f64().map(|t| t == *other).unwrap_or_default()
    }
}
