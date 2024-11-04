use std::num::ParseIntError;
use std::ops::{Add, Mul, MulAssign};
use std::str::FromStr;
use std::{cmp::Ordering, ops::AddAssign};

use serde::{de::Error as DeError, Deserialize, Serialize};

/// Represents the minimum occurrence of types or elements
pub type MinOccurs = usize;

/// Represents the maximum occurrence of types or elements
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MaxOccurs {
    /// The occurrence is unbounded.
    Unbounded,

    /// The occurrence is bound to the specified limit.
    Bounded(usize),
}

impl Add for MaxOccurs {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Bounded(a), Self::Bounded(b)) => Self::Bounded(a + b),
            (_, _) => Self::Unbounded,
        }
    }
}

impl AddAssign for MaxOccurs {
    fn add_assign(&mut self, rhs: Self) {
        match (&mut *self, rhs) {
            (Self::Bounded(a), Self::Bounded(b)) => *a += b,
            (_, _) => *self = Self::Unbounded,
        }
    }
}

impl Mul for MaxOccurs {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Bounded(a), Self::Bounded(b)) => Self::Bounded(a * b),
            (_, _) => Self::Unbounded,
        }
    }
}

impl MulAssign for MaxOccurs {
    fn mul_assign(&mut self, rhs: Self) {
        match (&mut *self, rhs) {
            (Self::Bounded(a), Self::Bounded(b)) => *a *= b,
            (_, _) => *self = Self::Unbounded,
        }
    }
}

impl Ord for MaxOccurs {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Bounded(a), Self::Bounded(b)) => a.cmp(b),
            (Self::Unbounded, Self::Unbounded) => Ordering::Equal,
            (Self::Bounded(_), Self::Unbounded) => Ordering::Less,
            (Self::Unbounded, Self::Bounded(_)) => Ordering::Greater,
        }
    }
}

impl PartialOrd for MaxOccurs {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Default for MaxOccurs {
    fn default() -> Self {
        Self::Bounded(1)
    }
}

impl FromStr for MaxOccurs {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "unbounded" {
            Ok(Self::Unbounded)
        } else {
            Ok(Self::Bounded(usize::from_str(s)?))
        }
    }
}

impl Serialize for MaxOccurs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Unbounded => serializer.serialize_str("unbounded"),
            Self::Bounded(x) => serializer.serialize_str(&format!("{x}")),
        }
    }
}

impl<'de> Deserialize<'de> for MaxOccurs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "unbounded" => Ok(Self::Unbounded),
            s => {
                Ok(Self::Bounded(s.parse().map_err(|_| {
                    DeError::custom("MaxOccurs. Invalid value!")
                })?))
            }
        }
    }
}
