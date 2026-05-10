use mirl_values_core::{
    settings::MapType,
    value::{
        AngleType, Color, ContainerValue, Datetime, LengthType, Number,
        SimpleValue, Time,
    },
};

use crate::{TryFromPatch, mod_and_pub_import};
mod_and_pub_import!(conversion);

// TODO: Auto impl some of these when Deref to Value is implemented
// TODO: Merge the Conainer + Simple + Super trait

/// What value should be used inside of [`ContainerValue`]
pub trait InnerCodecValue:
    std::fmt::Debug + Clone + PartialEq + PartialOrd + Ord
{
    /// The type used inside of [`ContainerValue`], it should be you Value Wrapper
    type Inner: std::fmt::Debug + Clone + PartialEq + PartialOrd + Ord;
}

/// Decompose a value by reference into its simple or container variant.
pub trait CodecSubValueRef:
    CodecContainerSubValueRef + CodecSimpleSubValueRef
// where
//     <<Self as CodecContainerSubValueRef>::InnerValue as InnerCodecValue>::Inner:
//         InnerCodecValue,
{
}
impl<T: CodecContainerSubValueRef + CodecSimpleSubValueRef> CodecSubValueRef
    for T
{
}

/// Decompose a value by ownership into its simple or container variant.
pub trait CodecSubValueInto:
    Sized + CodecContainerSubValueInto + CodecSimpleSubValueInto
{
}
/// Decompose a value by reference into its simple variant.
pub trait CodecSimpleSubValueRef {
    /// Your wrapper as a simple value
    fn as_simple(&self) -> Option<&SimpleValue>;
}
/// Decompose a value by reference into its simple variant.
pub trait CodecSimpleSubValueRefMut {
    /// Your wrapper as a simple value
    fn as_simple_mut(&mut self) -> Option<&mut SimpleValue>;
}

impl CodecSimpleSubValueRef for SimpleValue {
    fn as_simple(&self) -> Option<&SimpleValue> {
        Some(self)
    }
}
impl CodecSimpleSubValueRefMut for SimpleValue {
    fn as_simple_mut(&mut self) -> Option<&mut SimpleValue> {
        Some(self)
    }
}
/// Decompose a value by ownership into its simple or container variant.
pub trait CodecSimpleSubValueInto: Sized {
    /// Your wrapper as a simple value
    fn into_simple(self) -> Option<SimpleValue>;
}
impl CodecSimpleSubValueInto for SimpleValue {
    fn into_simple(self) -> Option<SimpleValue> {
        Some(self)
    }
}
/// Decompose a value by reference into its container variant.
pub trait CodecContainerSubValueRef {
    /// The [`InnerCodecValue`] for your Value wrapper
    type InnerValue: InnerCodecValue;
    /// Your wrapper as a container value
    fn as_container(
        &self,
    ) -> Option<&ContainerValue<<Self::InnerValue as InnerCodecValue>::Inner>>;
}
/// Decompose a value by reference into its container variant.
pub trait CodecContainerSubValueRefMut {
    /// The [`InnerCodecValue`] for your Value wrapper
    type InnerValue: InnerCodecValue;
    /// Your wrapper as a container value
    fn as_container_mut(
        &mut self,
    ) -> Option<&mut ContainerValue<<Self::InnerValue as InnerCodecValue>::Inner>>;
}
// TODO: Change stuff around so the impl below can be implemented
// impl<W: InnerCodecValue: MapType<W::Inner, W::Inner>>
//     CodecContainerSubValueRef for ContainerValue<W::Inner>
// {
//     type Inner = W;
//     fn as_container(&self) -> Option<&ContainerValue<Self::Inner>> {
//         Some(self)
//     }
// }

/// Decompose a value by ownership into its simple or container variant.
pub trait CodecContainerSubValueInto: Sized {
    /// The [`InnerCodecValue`] for your Value wrapper
    type InnerValue: InnerCodecValue;

    /// Your wrapper as a container value
    fn into_container(
        self,
    ) -> Option<ContainerValue<<Self::InnerValue as InnerCodecValue>::Inner>>;
}

// ── Everything below is auto-implemented ─────────────────────────────────────

/// Access simple (non-container) variants of a value by reference.
pub trait SimpleValueAccessRef: CodecSimpleSubValueRef {
    /// If the simple value is a string, return a reference to it
    fn as_string_ref(&self) -> Option<&String> {
        if let SimpleValue::String(v) = self.as_simple()? {
            Some(v)
        } else {
            None
        }
    }

    /// If the simple value is a bool, return a reference to it
    fn as_bool_ref(&self) -> Option<&bool> {
        if let SimpleValue::Bool(v) = self.as_simple()? {
            Some(v)
        } else {
            None
        }
    }

    /// If the simple value is a number, return a reference to it
    fn as_number_ref(&self) -> Option<&Number> {
        if let SimpleValue::Number(v) = self.as_simple()? {
            Some(v)
        } else {
            None
        }
    }

    /// If the simple value is a time, return a reference to it
    fn as_time_ref(&self) -> Option<&Time> {
        if let SimpleValue::Time(v) = self.as_simple()? {
            Some(v)
        } else {
            None
        }
    }

    /// If the simple value is a datetime, return a reference to it
    fn as_datetime_ref(&self) -> Option<&Datetime> {
        if let SimpleValue::DateTime(v) = self.as_simple()? {
            Some(v)
        } else {
            None
        }
    }

    /// If the simple value is an angle, return a reference to it
    fn as_angle_ref(&self) -> Option<(&Number, &AngleType)> {
        if let SimpleValue::Angle(v, angle) = self.as_simple()? {
            Some((v, angle))
        } else {
            None
        }
    }

    /// If the simple value is a literal, return a reference to it
    fn as_literal_ref(&self) -> Option<&String> {
        if let SimpleValue::Literal(v) = self.as_simple()? {
            Some(v)
        } else {
            None
        }
    }

    /// If the simple value is a length, return a reference to it
    fn as_length_ref(&self) -> Option<(&Number, &LengthType)> {
        if let SimpleValue::Length(v, l) = self.as_simple()? {
            Some((v, l))
        } else {
            None
        }
    }

    /// If the simple value is a color, return a reference to it
    fn as_color_ref(&self) -> Option<&Color> {
        if let SimpleValue::Color(v) = self.as_simple()? {
            Some(v)
        } else {
            None
        }
    }

    /// If the simple value is bytes, return a reference to it
    fn as_bytes_ref(&self) -> Option<(&Vec<u8>, &String)> {
        if let SimpleValue::Bytes(v, e) = self.as_simple()? {
            Some((v, e))
        } else {
            None
        }
    }

    /// If the simple value is none, return `Some(None)`
    fn as_none(&self) -> Option<Option<()>> {
        if matches!(self.as_simple()?, SimpleValue::None) {
            Some(None)
        } else {
            None
        }
    }
}
/// Access simple (non-container) variants of a value by reference.
pub trait SimpleValueAccessRefMut: CodecSimpleSubValueRefMut {
    /// If the simple value is a string, return a reference to it
    fn as_string_ref_mut(&mut self) -> Option<&mut String> {
        if let SimpleValue::String(v) = self.as_simple_mut()? {
            Some(v)
        } else {
            None
        }
    }

    /// If the simple value is a bool, return a reference to it
    fn as_bool_ref_mut(&mut self) -> Option<&mut bool> {
        if let SimpleValue::Bool(v) = self.as_simple_mut()? {
            Some(v)
        } else {
            None
        }
    }

    /// If the simple value is a number, return a reference to it
    fn as_number_ref_mut(&mut self) -> Option<&mut Number> {
        if let SimpleValue::Number(v) = self.as_simple_mut()? {
            Some(v)
        } else {
            None
        }
    }

    /// If the simple value is a time, return a reference to it
    fn as_time_ref_mut(&mut self) -> Option<&mut Time> {
        if let SimpleValue::Time(v) = self.as_simple_mut()? {
            Some(v)
        } else {
            None
        }
    }

    /// If the simple value is a datetime, return a reference to it
    fn as_datetime_ref_mut(&mut self) -> Option<&mut Datetime> {
        if let SimpleValue::DateTime(v) = self.as_simple_mut()? {
            Some(v)
        } else {
            None
        }
    }

    /// If the simple value is an angle, return a reference to it
    fn as_angle_ref_mut(&mut self) -> Option<(&mut Number, &mut AngleType)> {
        if let SimpleValue::Angle(v, angle) = self.as_simple_mut()? {
            Some((v, angle))
        } else {
            None
        }
    }

    /// If the simple value is a literal, return a reference to it
    fn as_literal_ref_mut(&mut self) -> Option<&mut String> {
        if let SimpleValue::Literal(v) = self.as_simple_mut()? {
            Some(v)
        } else {
            None
        }
    }

    /// If the simple value is a length, return a reference to it
    fn as_length_ref_mut(&mut self) -> Option<(&mut Number, &mut LengthType)> {
        if let SimpleValue::Length(v, l) = self.as_simple_mut()? {
            Some((v, l))
        } else {
            None
        }
    }

    /// If the simple value is a color, return a reference to it
    fn as_color_ref_mut(&mut self) -> Option<&mut Color> {
        if let SimpleValue::Color(v) = self.as_simple_mut()? {
            Some(v)
        } else {
            None
        }
    }

    /// If the simple value is bytes, return a reference to it
    fn as_bytes_ref_mut(&mut self) -> Option<(&mut Vec<u8>, &mut String)> {
        if let SimpleValue::Bytes(v, e) = self.as_simple_mut()? {
            Some((v, e))
        } else {
            None
        }
    }
}

// TODO: Implement this automatically
/// Access simple (non-container) variants of a value by reference.
pub trait SimpleValueAccessToNumber: CodecSimpleSubValueRef {
    /// If the simple value is a number, return it in the selected number format if possible
    fn to_number<
        T: TryFromPatch<i128>
            + TryFromPatch<f64>
            + TryFromPatch<num_bigint::BigInt>
            + TryFromPatch<num_bigfloat::BigFloat>,
    >(
        &self,
    ) -> Option<T> {
        if let SimpleValue::Number(v) = self.as_simple()? {
            Some(v.clone().to_number()?)
        } else {
            None
        }
    }

    /// If the simple value is a number, return it in the legacy number format if possible
    fn to_number_legacy<T: TryFrom<Number>>(&self) -> Option<T> {
        if let SimpleValue::Number(v) = self.as_simple()? {
            T::try_from(v.clone()).ok()
        } else {
            None
        }
    }
}
// Blanket impl: anything that can be inspected by ref gets all simple ref accessors for free.
impl<T: CodecSimpleSubValueRef> SimpleValueAccessRef for T {}

impl<T: CodecSimpleSubValueRef> SimpleValueAccessToNumber for T {}

/// Access container (vec/map) variants of a value by reference.
pub trait ContainerValueAccessRef: CodecContainerSubValueRef {
    /// If the container value is a vec, return a reference to it
    fn as_vec_ref(
        &self,
    ) -> Option<&Vec<<Self::InnerValue as InnerCodecValue>::Inner>> {
        if let ContainerValue::Vec(v) = self.as_container()? {
            Some(v)
        } else {
            None
        }
    }

    /// If the container value is a map, return a reference to it
    #[allow(clippy::type_complexity)]
    fn as_map_ref(
        &self,
    ) -> Option<
        &MapType<
            <Self::InnerValue as InnerCodecValue>::Inner,
            <Self::InnerValue as InnerCodecValue>::Inner,
        >,
    > {
        if let ContainerValue::Map(v) = self.as_container()? {
            Some(v)
        } else {
            None
        }
    }
}

/// Access container (vec/map) variants of a value by reference.
pub trait ContainerValueAccessRefMut: CodecContainerSubValueRefMut {
    /// If the container value is a vec, return a reference to it
    fn as_vec_ref_mut(
        &mut self,
    ) -> Option<&mut Vec<<Self::InnerValue as InnerCodecValue>::Inner>> {
        if let ContainerValue::Vec(v) = self.as_container_mut()? {
            Some(v)
        } else {
            None
        }
    }

    /// If the container value is a map, return a reference to it
    #[allow(clippy::type_complexity)]
    fn as_map_ref_mut(
        &mut self,
    ) -> Option<
        &mut MapType<
            <Self::InnerValue as InnerCodecValue>::Inner,
            <Self::InnerValue as InnerCodecValue>::Inner,
        >,
    > {
        if let ContainerValue::Map(v) = self.as_container_mut()? {
            Some(v)
        } else {
            None
        }
    }
}

// Blanket impl: anything that can be inspected by ref gets all container ref accessors for free.
impl<T: CodecContainerSubValueRef> ContainerValueAccessRef for T {}
// Blanket impl: anything that can be inspected by ref gets all container ref accessors for free.
impl<T: CodecContainerSubValueRefMut> ContainerValueAccessRefMut for T {}

/// Master by-reference accessor trait combining simple and container access.
///
/// Prefer bounding on this trait when you need access to all value variants.
/// Both [`SimpleValueAccessRef`] and [`ContainerValueAccessRef`] are available
/// on any type that implements this trait.
pub trait CodecValueAccessRef:
    SimpleValueAccessRef + ContainerValueAccessRef
{
}

// Blanket impl: everything that satisfies both sub-traits gets the master trait for free.
impl<T: SimpleValueAccessRef + ContainerValueAccessRef> CodecValueAccessRef
    for T
{
}
/// Master by-reference accessor trait combining simple and container access.
///
/// Prefer bounding on this trait when you need access to all value variants.
/// Both [`SimpleValueAccessRefMut`] and [`ContainerValueAccessRefMut`] are available
/// on any type that implements this trait.
pub trait CodecValueAccessRefMut:
    SimpleValueAccessRefMut + ContainerValueAccessRefMut
{
}
impl<T: SimpleValueAccessRefMut + ContainerValueAccessRefMut>
    CodecValueAccessRefMut for T
{
}
impl<T: CodecSimpleSubValueRefMut> SimpleValueAccessRefMut for T {}

/// Access simple (non-container) variants of a value by ownership.
pub trait SimpleValueAccessInto: CodecSimpleSubValueInto {
    /// Consume self and return a string if Self is a string
    fn into_string(self) -> Option<String> {
        if let SimpleValue::String(v) = self.into_simple()? {
            Some(v)
        } else {
            None
        }
    }

    /// Consume self and return a bool if Self is a bool
    fn into_bool(self) -> Option<bool> {
        if let SimpleValue::Bool(v) = self.into_simple()? {
            Some(v)
        } else {
            None
        }
    }

    /// Consume self and return a number if Self is a number
    fn into_number(self) -> Option<Number> {
        if let SimpleValue::Number(v) = self.into_simple()? {
            Some(v)
        } else {
            None
        }
    }

    /// Consume self and return a time if Self is a time
    fn into_time(self) -> Option<Time> {
        if let SimpleValue::Time(v) = self.into_simple()? {
            Some(v)
        } else {
            None
        }
    }

    /// Consume self and return a datetime if Self is a datetime
    fn into_datetime(self) -> Option<Datetime> {
        if let SimpleValue::DateTime(v) = self.into_simple()? {
            Some(v)
        } else {
            None
        }
    }

    /// Consume self and return an angle if Self is an angle
    fn into_angle(self) -> Option<(Number, AngleType)> {
        if let SimpleValue::Angle(v, a) = self.into_simple()? {
            Some((v, a))
        } else {
            None
        }
    }

    /// Consume self and return a literal if Self is a literal
    fn into_literal(self) -> Option<String> {
        if let SimpleValue::Literal(v) = self.into_simple()? {
            Some(v)
        } else {
            None
        }
    }

    /// Consume self and return a length if Self is a length
    fn into_length(self) -> Option<(Number, LengthType)> {
        if let SimpleValue::Length(v, l) = self.into_simple()? {
            Some((v, l))
        } else {
            None
        }
    }

    /// Consume self and return a color if Self is a color
    fn into_color(self) -> Option<Color> {
        if let SimpleValue::Color(v) = self.into_simple()? {
            Some(v)
        } else {
            None
        }
    }

    /// Consume self and return bytes if Self is bytes
    fn into_bytes(self) -> Option<(Vec<u8>, String)> {
        if let SimpleValue::Bytes(v, e) = self.into_simple()? {
            Some((v, e))
        } else {
            None
        }
    }
}

// Blanket impl: anything that can be consumed gets all simple owned accessors for free.
impl<T: CodecSimpleSubValueInto> SimpleValueAccessInto for T {}

/// Access container (vec/map) variants of a value by ownership.
pub trait ContainerValueAccessInto: CodecContainerSubValueInto {
    /// Consume self and return a vec if Self is a vec
    fn into_vec(
        self,
    ) -> Option<Vec<<Self::InnerValue as InnerCodecValue>::Inner>> {
        if let ContainerValue::Vec(v) = self.into_container()? {
            Some(v)
        } else {
            None
        }
    }

    #[allow(clippy::type_complexity)]
    /// Consume self and return a map if Self is a map
    fn into_map(
        self,
    ) -> Option<
        MapType<
            <Self::InnerValue as InnerCodecValue>::Inner,
            <Self::InnerValue as InnerCodecValue>::Inner,
        >,
    > {
        if let ContainerValue::Map(v) = self.into_container()? {
            Some(v)
        } else {
            None
        }
    }

    /// Consume self and convert the map if self is a map
    fn to_map<
        T: TryFromPatch<
            MapType<
                <Self::InnerValue as InnerCodecValue>::Inner,
                <Self::InnerValue as InnerCodecValue>::Inner,
            >,
        >,
    >(
        self,
    ) -> Option<T> {
        T::try_from_value(self.into_map()?)
    }
}

// Blanket impl: anything that can be consumed gets all container owned accessors for free.
impl<T: CodecContainerSubValueInto> ContainerValueAccessInto for T {}

/// Master by-ownership accessor trait combining simple and container access.
///
/// Prefer bounding on this trait when you need to consume all value variants.
/// Both [`SimpleValueAccessInto`] and [`ContainerValueAccessInto`] are available
/// on any type that implements this trait.
pub trait ValueAccessInto:
    SimpleValueAccessInto + ContainerValueAccessInto
{
}

// Blanket impl: everything that satisfies both sub-traits gets the master trait for free.
impl<T: SimpleValueAccessInto + ContainerValueAccessInto> ValueAccessInto
    for T
{
}
