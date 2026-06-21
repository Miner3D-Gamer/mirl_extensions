use std::hint::unreachable_unchecked;

use mirl_values_core::value::{Number, ValueType};

use crate::{CodecSubValueRef, Fract, TryFromPatch, TryIntoPatch};

/// Convert the number into a number
pub trait ValueNumberIntoNumberValue<
    T: TryFromPatch<i128>
        + TryFromPatch<f64>
        + TryFromPatch<num_bigint::BigInt>
        + TryFromPatch<num_bigfloat::BigFloat>,
>
{
    /// Convert the number into a more normal number if Number fits into the value
    fn to_number(self) -> Option<T>;
}

impl<
    T: TryFromPatch<i128>
        + TryFromPatch<f64>
        + TryFromPatch<num_bigint::BigInt>
        + TryFromPatch<num_bigfloat::BigFloat>,
> ValueNumberIntoNumberValue<T> for Number
{
    fn to_number(self) -> Option<T> {
        match self {
            Self::Int(v) => T::try_from_value(v),
            Self::Float(v) => T::try_from_value(v),
            Self::BigFloat(v) => T::try_from_value(v),
            Self::BigInt(v) => T::try_from_value(v),
        }
    }
}
/// [`simplify`](Self::simplify)
pub trait SimplifyValueNumber {
    /// Try to convert the current value into a smaller one without loosing fidelity
    #[must_use]
    fn simplify(self) -> Self;
}

impl SimplifyValueNumber for Number {
    fn simplify(self) -> Self {
        use std::str::FromStr;
        match self {
            Self::Int(val) => Self::Int(val),
            Self::BigInt(val) => val
                .clone()
                .try_into_value()
                .map_or(Self::BigInt(val), Self::Int),
            Self::BigFloat(val) => {
                if val.is_nan() {
                    return Self::Float(f64::NAN);
                }
                if val.fract().is_zero() {
                    num_bigint::BigInt::from_str(&val.to_string()).ok().map_or(
                        Self::BigFloat(val),
                        |val| {
                            val.clone()
                                .try_into_value()
                                .map_or(Self::BigInt(val), Self::Int)
                        },
                    )
                } else {
                    let value = val.to_f64();
                    if value.is_finite() {
                        Self::Float(value)
                    } else {
                        Self::BigFloat(val)
                    }
                }
            }
            Self::Float(val) => val.try_into_value().map_or(Self::Float(val), Self::Int),
        }
    }
}

// // TODO: Uncomment once specialization is available
// impl<T, V> TryFromPatch<V> for T
// where
//     T: CodecValueAccessRef
//         + FromPatch<i128>
//         + FromPatch<f64>
//         + FromPatch<BigFloat>
//         + FromPatch<BigInt>,
//     V: IntoPatch<T>,
// {
//     fn try_from_value(value: V) -> Option<Self> {
//         value.into_value().to_number()
//     }
// }

/// Get the value type of a value
pub trait GetCodecValueType {
    #[must_use]
    /// Get the value type of a value
    fn get_value_type(&self) -> ValueType;
}
impl<T: CodecSubValueRef> GetCodecValueType for T {
    fn get_value_type(&self) -> ValueType {
        self.as_simple().map_or_else(
            || {
                self.as_container().map_or_else(
                    || unsafe { unreachable_unchecked() },
                    mirl_values_core::value::ContainerValue::get_value_type,
                )
            },
            mirl_values_core::value::SimpleValue::get_value_type,
        )
    }
}
impl GetCodecValueType for mirl_values_core::value::SimpleValue {
    fn get_value_type(&self) -> ValueType {
        self.get_value_type()
    }
}
