use std::{borrow::Cow, fmt::Display};

/// The Val trait may be implemented to represent values to be
/// (de)serialized with WAVE, notably [`wasmtime::component::Val`].
///
/// The `make_*` and `unwrap_*` methods should be called only for corresponding
/// [`crate::ty::Kind`]s.
#[allow(unused_variables)]
pub trait Val: Clone + Sized {
    /// A type representing types of these values.
    type Type: crate::ty::Type;

    /// An associated error which can be returned from creating values.
    type Error: Display;

    /// The type of this value.
    fn ty(&self) -> Self::Type;

    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_bool(val: bool) -> Self {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_s8(val: i8) -> Self {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_s16(val: i16) -> Self {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_s32(val: i32) -> Self {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_s64(val: i64) -> Self {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_u8(val: u8) -> Self {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_u16(val: u16) -> Self {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_u32(val: u32) -> Self {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_u64(val: u64) -> Self {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_float32(val: f32) -> Self {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_float64(val: f64) -> Self {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_char(val: char) -> Self {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_string(val: Cow<str>) -> Self {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_list(
        ty: &Self::Type,
        vals: impl IntoIterator<Item = Self>,
    ) -> Result<Self, Self::Error> {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_record<'a>(
        ty: &Self::Type,
        fields: impl IntoIterator<Item = (&'a str, Self)>,
    ) -> Result<Self, Self::Error> {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_tuple(
        ty: &Self::Type,
        vals: impl IntoIterator<Item = Self>,
    ) -> Result<Self, Self::Error> {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_variant(ty: &Self::Type, case: &str, val: Option<Self>) -> Result<Self, Self::Error> {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_enum(ty: &Self::Type, case: &str) -> Result<Self, Self::Error> {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_option(ty: &Self::Type, val: Option<Self>) -> Result<Self, Self::Error> {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_result(
        ty: &Self::Type,
        val: Result<Option<Self>, Option<Self>>,
    ) -> Result<Self, Self::Error> {
        unimplemented!()
    }
    /// Returns a new Val of the given type.
    /// # Panics
    /// The default implementation panics (unimplemented).
    fn make_flags<'a>(
        ty: &Self::Type,
        names: impl IntoIterator<Item = &'a str>,
    ) -> Result<Self, Self::Error> {
        unimplemented!()
    }

    /// Returns the underlying value of the Val, panicing if it's the wrong type.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_bool(&self) -> bool {
        unimplemented!()
    }
    /// Returns the underlying value of the Val, panicing if it's the wrong type.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_s8(&self) -> i8 {
        unimplemented!()
    }
    /// Returns the underlying value of the Val, panicing if it's the wrong type.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_s16(&self) -> i16 {
        unimplemented!()
    }
    /// Returns the underlying value of the Val, panicing if it's the wrong type.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_s32(&self) -> i32 {
        unimplemented!()
    }
    /// Returns the underlying value of the Val, panicing if it's the wrong type.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_s64(&self) -> i64 {
        unimplemented!()
    }
    /// Returns the underlying value of the Val, panicing if it's the wrong type.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_u8(&self) -> u8 {
        unimplemented!()
    }
    /// Returns the underlying value of the Val, panicing if it's the wrong type.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_u16(&self) -> u16 {
        unimplemented!()
    }
    /// Returns the underlying value of the Val, panicing if it's the wrong type.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_u32(&self) -> u32 {
        unimplemented!()
    }
    /// Returns the underlying value of the Val, panicing if it's the wrong type.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_u64(&self) -> u64 {
        unimplemented!()
    }
    /// Returns the underlying value of the Val, panicing if it's the wrong type.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_float32(&self) -> f32 {
        unimplemented!()
    }
    /// Returns the underlying value of the Val, panicing if it's the wrong type.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_float64(&self) -> f64 {
        unimplemented!()
    }
    /// Returns the underlying value of the Val, panicing if it's the wrong type.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_char(&self) -> char {
        unimplemented!()
    }
    /// Returns the underlying value of the Val, panicing if it's the wrong type.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_string(&self) -> Cow<str> {
        unimplemented!()
    }
    /// Returns an iterator of the element Vals of the list.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_list(&self) -> Box<dyn Iterator<Item = Cow<Self>> + '_> {
        unimplemented!()
    }
    /// Returns an iterator of the field names and Vals of the record.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_record(&self) -> Box<dyn Iterator<Item = (Cow<str>, Cow<Self>)> + '_> {
        unimplemented!()
    }
    /// Returns an iterator of the field Vals of the tuple.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_tuple(&self) -> Box<dyn Iterator<Item = Cow<Self>> + '_> {
        unimplemented!()
    }
    /// Returns the variant case name and optional payload Val of the variant.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_variant(&self) -> (Cow<str>, Option<Cow<Self>>) {
        unimplemented!()
    }
    /// Returns the case name of the enum.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_enum(&self) -> Cow<str> {
        unimplemented!()
    }
    /// Returns the optional Val.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_option(&self) -> Option<Cow<Self>> {
        unimplemented!()
    }
    /// Returns Ok(_) or Err(_) with the optional payload Val.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_result(&self) -> Result<Option<Cow<Self>>, Option<Cow<Self>>> {
        unimplemented!()
    }
    /// Returns an iterator of the names of the flags Val.
    /// # Panics
    /// Panics if `self` is not of the right type.
    fn unwrap_flags(&self) -> Box<dyn Iterator<Item = Cow<str>> + '_> {
        unimplemented!()
    }
}

macro_rules! unwrap_val {
    ($self:ident, $case:path, $name:expr) => {
        match $self {
            $case(v) => v,
            _ => panic!("called unwrap_{name} on non-{name} value", name = $name),
        }
    };
}

pub(crate) use unwrap_val;
