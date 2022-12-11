use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use std::str::FromStr;
use crate::Primitive;

impl<T: Primitive> Primitive for Range<T> {
    fn stringify(&self) -> String {
        format!("{}..{}", self.start.stringify(), self.end.stringify())
    }

    fn from_string(string: &str) -> Option<Self>{
        let parts = string.split_once("..")?;
        Some(T::from_string(parts.0)?..T::from_string(parts.1)?)
    }
}

impl<T: Primitive> Primitive for RangeFrom<T> {
    fn stringify(&self) -> String {
        format!("{}..", self.start.stringify())
    }

    fn from_string(string: &str) -> Option<Self>{
        let parts = string.split_once("..")?;
        Some(T::from_string(parts.0)?..)
    }
}

impl Primitive for RangeFull {
    fn stringify(&self) -> String {
        String::from("..")
    }

    fn from_string(_string: &str) -> Option<Self>{
        Some(RangeFull)
    }
}

impl<T: Primitive> Primitive for RangeInclusive<T> {
    fn stringify(&self) -> String {
        format!("{}..={}", self.start().stringify(), self.end().stringify())
    }

    fn from_string(string: &str) -> Option<Self>{
        let parts = string.split_once("..=")?;
        Some(T::from_string(parts.0)?..=T::from_string(parts.1)?)
    }
}

impl<T: Primitive> Primitive for RangeTo<T> {
    fn stringify(&self) -> String {
        format!("..{}", self.end.stringify())
    }

    fn from_string(string: &str) -> Option<Self>{
        let parts = string.split_once("..")?;
        Some(..T::from_string(parts.0)?)
    }
}

impl<T: Primitive> Primitive for RangeToInclusive<T> {
    fn stringify(&self) -> String {
        format!("..={}", self.end.stringify())
    }

    fn from_string(string: &str) -> Option<Self>{
        let parts = string.split_once("..=")?;
        Some(..=T::from_string(parts.0)?)
    }
}

pub enum AnyRange<T> {
    Range(Range<T>),
    RangeFrom(RangeFrom<T>),
    RangeFull(RangeFull),
    RangeInclusive(RangeInclusive<T>),
    RangeTo(RangeTo<T>),
    RangeToInclusive(RangeToInclusive<T>),
}

impl<T> From<Range<T>> for AnyRange<T>{
    fn from(value: Range<T>) -> Self {
        AnyRange::Range(value)
    }
}
impl<T> From<RangeFrom<T>> for AnyRange<T>{
    fn from(value: RangeFrom<T>) -> Self {
        AnyRange::RangeFrom(value)
    }
}
impl<T> From<RangeFull> for AnyRange<T>{
    fn from(value: RangeFull) -> Self {
        AnyRange::RangeFull(value)
    }
}
impl<T> From<RangeInclusive<T>> for AnyRange<T>{
    fn from(value: RangeInclusive<T>) -> Self {
        AnyRange::RangeInclusive(value)
    }
}
impl<T> From<RangeTo<T>> for AnyRange<T>{
    fn from(value: RangeTo<T>) -> Self {
        AnyRange::RangeTo(value)
    }
}
impl<T> From<RangeToInclusive<T>> for AnyRange<T>{
    fn from(value: RangeToInclusive<T>) -> Self {
        AnyRange::RangeToInclusive(value)
    }
}

impl<T: Primitive> Primitive for AnyRange<T>{
    fn stringify(&self) -> String {
        match self {
            AnyRange::Range(r) => r.stringify(),
            AnyRange::RangeFrom(r) => r.stringify(),
            AnyRange::RangeFull(r) => r.stringify(),
            AnyRange::RangeInclusive(r) => r.stringify(),
            AnyRange::RangeTo(r) => r.stringify(),
            AnyRange::RangeToInclusive(r) => r.stringify()
        }
    }

    fn from_string(string: &str) -> Option<Self> where Self: Sized{
        match 0 {
            _ if let Some(r) = Range::<T>::from_string(string) => Some(AnyRange::Range(r)),
            _ if let Some(r) = RangeFrom::<T>::from_string(string) => Some(AnyRange::RangeFrom(r)),
            _ if let Some(r) = RangeFull::from_string(string) => Some(AnyRange::RangeFull(r)),
            _ if let Some(r) = RangeInclusive::<T>::from_string(string) => Some(AnyRange::RangeInclusive(r)),
            _ if let Some(r) = RangeTo::<T>::from_string(string) => Some(AnyRange::RangeTo(r)),
            _ if let Some(r) = RangeToInclusive::<T>::from_string(string) => Some(AnyRange::RangeToInclusive(r)),
            _ => None
        }
    }
}

impl<T: Clone + PartialOrd + PartialEq> RangeBounds<T> for AnyRange<T> {
    fn start_bound(&self) -> Bound<&T> {
        match self {
            Self::Range(r) => r.start_bound(),
            Self::RangeFrom(r) => r.start_bound(),
            Self::RangeFull(r) => r.start_bound(),
            Self::RangeInclusive(r) => r.start_bound(),
            Self::RangeTo(r) => r.start_bound(),
            Self::RangeToInclusive(r) => r.start_bound(),
        }
    }
    fn end_bound(&self) -> Bound<&T> {
        match self {
            Self::Range(r) => r.end_bound(),
            Self::RangeFrom(r) => r.end_bound(),
            Self::RangeFull(r) => r.end_bound(),
            Self::RangeInclusive(r) => r.end_bound(),
            Self::RangeTo(r) => r.end_bound(),
            Self::RangeToInclusive(r) => r.end_bound(),
        }
    }
}

impl<T: Primitive> Debug for AnyRange<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stringify())
    }
}
