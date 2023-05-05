//! The IMAP ENABLE Extension
//!
//! This extension extends ...
//!
//! * the [Capability](crate::response::Capability) enum with a new variant [Capability::Enable](crate::response::Capability#variant.Enable),
//! * the [CommandBody](crate::command::CommandBody) enum with a new variant [CommandBody::Enable](crate::command::CommandBody#variant.Enable), and
//! * the [Data](crate::response::Data) enum with a new variant [Data::Enabled](crate::response::Data#variant.Enabled).

use std::{
    convert::{TryFrom, TryInto},
    io::Write,
};

#[cfg(feature = "arbitrary")]
use arbitrary::Arbitrary;
#[cfg(feature = "bounded-static")]
use bounded_static::ToStatic;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    codec::Encode,
    command::CommandBody,
    core::{Atom, NonEmptyVec},
    response::Data,
};

impl<'a> CommandBody<'a> {
    pub fn enable<C>(capabilities: C) -> Result<Self, C::Error>
    where
        C: TryInto<NonEmptyVec<CapabilityEnable<'a>>>,
    {
        Ok(CommandBody::Enable {
            capabilities: capabilities.try_into()?,
        })
    }
}

impl<'a> Data<'a> {
    // TODO
    // pub fn enable() -> Self {
    //     unimplemented!()
    // }
}

#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(feature = "bounded-static", derive(ToStatic))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CapabilityEnable<'a> {
    Utf8(Utf8Kind),
    Other(CapabilityEnableOther<'a>),
}

impl<'a> From<Atom<'a>> for CapabilityEnable<'a> {
    fn from(atom: Atom<'a>) -> Self {
        match atom.as_ref().to_ascii_lowercase().as_ref() {
            "utf8=accept" => Self::Utf8(Utf8Kind::Accept),
            "utf8=only" => Self::Utf8(Utf8Kind::Only),
            _ => Self::Other(CapabilityEnableOther(atom)),
        }
    }
}

#[cfg_attr(feature = "bounded-static", derive(ToStatic))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapabilityEnableOther<'a>(Atom<'a>);

impl<'a> TryFrom<Atom<'a>> for CapabilityEnableOther<'a> {
    type Error = CapabilityEnableOtherError;

    fn try_from(value: Atom<'a>) -> Result<Self, Self::Error> {
        match value.as_ref().to_ascii_lowercase().as_ref() {
            "utf8=accept" | "utf8=only" => Err(CapabilityEnableOtherError::Reserved),
            _ => Ok(Self(value)),
        }
    }
}

#[derive(Clone, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum CapabilityEnableOtherError {
    #[error("Please use one of the supported variants.")]
    Reserved,
}

#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(feature = "bounded-static", derive(ToStatic))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Utf8Kind {
    Accept,
    Only,
}

impl<'a> Encode for CapabilityEnable<'a> {
    fn encode(&self, writer: &mut impl Write) -> std::io::Result<()> {
        match self {
            Self::Utf8(Utf8Kind::Accept) => writer.write_all(b"UTF8=ACCEPT"),
            Self::Utf8(Utf8Kind::Only) => writer.write_all(b"UTF8=ONLY"),
            Self::Other(other) => other.encode(writer),
        }
    }
}

impl<'a> Encode for CapabilityEnableOther<'a> {
    fn encode(&self, writer: &mut impl Write) -> std::io::Result<()> {
        self.0.encode(writer)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::*;
    use crate::imap4rev1::core::NonEmptyVecError;

    #[test]
    fn test_encode_command_body_enable() {
        let tests = [
            (
                CommandBody::enable(vec![CapabilityEnable::Utf8(Utf8Kind::Only)]),
                Ok((
                    CommandBody::Enable {
                        capabilities: NonEmptyVec::try_from(vec![CapabilityEnable::Utf8(
                            Utf8Kind::Only,
                        )])
                        .unwrap(),
                    },
                    b"ENABLE UTF8=ONLY".as_ref(),
                )),
            ),
            (
                CommandBody::enable(vec![CapabilityEnable::Utf8(Utf8Kind::Accept)]),
                Ok((
                    CommandBody::Enable {
                        capabilities: NonEmptyVec::try_from(vec![CapabilityEnable::Utf8(
                            Utf8Kind::Accept,
                        )])
                        .unwrap(),
                    },
                    b"ENABLE UTF8=ACCEPT",
                )),
            ),
            (
                CommandBody::enable(vec![CapabilityEnable::Other(
                    CapabilityEnableOther::try_from(Atom::try_from("FOO").unwrap()).unwrap(),
                )]),
                Ok((
                    CommandBody::Enable {
                        capabilities: NonEmptyVec::try_from(vec![CapabilityEnable::Other(
                            CapabilityEnableOther::try_from(Atom::try_from("FOO").unwrap())
                                .unwrap(),
                        )])
                        .unwrap(),
                    },
                    b"ENABLE FOO",
                )),
            ),
            (CommandBody::enable(vec![]), Err(NonEmptyVecError::Empty)),
        ];

        for (test, expected) in tests {
            match test {
                Ok(got) => {
                    let bytes = got.encode_detached().unwrap();
                    assert_eq!(expected, Ok((got, bytes.as_ref())));
                }
                Err(got) => {
                    assert_eq!(Err(got), expected);
                }
            }
        }
    }

    #[test]
    fn test_conversion_capability_enable_other() {
        assert_eq!(
            CapabilityEnable::from(Atom::try_from("utf8=only").unwrap()),
            CapabilityEnable::Utf8(Utf8Kind::Only)
        );
        assert_eq!(
            CapabilityEnable::from(Atom::try_from("utf8=accept").unwrap()),
            CapabilityEnable::Utf8(Utf8Kind::Accept)
        );
        assert_eq!(
            CapabilityEnableOther::try_from(Atom::try_from("utf8=only").unwrap()),
            Err(CapabilityEnableOtherError::Reserved)
        );
        assert_eq!(
            CapabilityEnableOther::try_from(Atom::try_from("utf8=accept").unwrap()),
            Err(CapabilityEnableOtherError::Reserved)
        );
    }
}
