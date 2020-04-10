// LNP/BP Rust Library
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.


use core::slice::Iter;
use std::convert::TryFrom;
use std::marker::PhantomData;
use zmq::Message;

use bitcoin::consensus::encode::{
    serialize as consensus_serialize,
    deserialize as consensus_deserialize
};

use crate::bp;
#[cfg(feature="use-rgb")]
use crate::rgb;
#[cfg(feature="use-rgb")]
use crate::csv::{network_serialize, network_deserialize};
use super::{Multipart, Error};


// 1. Encoding messages
pub trait MessageEncode<T> {
    type Error: std::error::Error;
    fn into_message(self) -> Message;
    fn try_from_message(message: Message) -> Result<Self, Self::Error>;
}

// 1.1. Auto impl for types defining own Message serialization rules with TryFrom/Into
impl<T> MessageEncode<T> for T where T: TryFrom<Message, Error = Error> + Into<Message> {
    type Error = Error;
    fn into_message(self) -> Message {
        self.into()
    }
    fn try_from_message(message: Message) -> Result<Self, Self::Error> {
        Self::try_from(message)
    }
}

// 1.2. Auto impl for bitcoin-serialized types
impl<T> From<T> for Message where T: bp::marker::BitcoinData {
    fn from(data: T) -> Message {
        Message::from(consensus_serialize(data))
    }
}

impl<T> TryFrom<Message> for T where T: bp::marker::BitcoinData {
    type Error = Error;
    fn try_from(message: Message) -> Result<Self, Self::Error> {
        Ok(consensus_deserialize(&message)?)
    }
}

// 1.3. Auto impl for client-validation-serialized types
#[cfg(feature="use-rgb")]
impl<T> From<T> for Message where T: rgb::marker::RgbData {
    fn from(data: T) -> Message {
        Message::from(network_serialize(data)
            .expect("Commitment serialize failed"))
    }
}

#[cfg(feature="use-rgb")]
impl<T> TryFrom<Message> for T where T: rgb::marker::RgbData {
    type Error = Error;
    fn try_from(message: Message) -> Result<T, Self::Error> {
        Ok(network_deserialize(&message)?)
    }
}

/*
// 1.4. Impl for bp::ShortId
impl MessageEncode for ShortId {
    type Error = Error;
    fn into_message(self) -> Message {
        Message::from(self.into_u64())
    }
    fn try_from_message(message: Message) -> Result<Self, Self::Error> {
        Self::try_from(&message)
    }
}
*/


// 2. Encoding multipart messages
pub trait MultipartEncode<'a, T>: TryFrom<&'a [Message]> + Into<Multipart> {
    fn into_multipart(self) -> Multipart {
        self.into()
    }
}

// Primitive type implementations
// 1. Vector
//wrapper!(ReqVec<'a, T: ReqArg<'a>>, PhantomData<&'a Vec<T>>, Vec<T>,
//         doc="Wrapper around `Vec` supporting `Req` trait");
#[repr(transparent)]
pub struct VecEncoding<T: MessageEncode<T>>(Vec<T>);

// repr(transparent) is not yet working for generics, so we have to implement manually
impl<T> VecEncoding<T> where T: MessageEncode<T> {
    pub fn new(vec: Vec<T>) -> Self { Self(vec, PhantomData::default()) }
    pub fn iter(&self) -> Iter<T> { self.0.iter() }
}

// repr(transparent) is not yet working for generics, so we have to implement manually
impl<T> IntoIterator for VecEncoding<T> where T: MessageEncode<T> {
    type Item = <Vec<T> as IntoIterator>::Item;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<T> MultipartEncode<'_, T> for VecEncoding<T> where T: MessageEncode<T> { }

impl<T> TryFrom<&[Message]> for VecEncoding<T> where T: MessageEncode<T> {
    type Error = ();

    fn try_from(args: &[Message]) -> Result<Self, Self::Error> {
        Ok(VecEncoding::new(args.iter().try_fold(Vec::<T>::new(), |mut vec, arg| {
            vec.push(T::try_from(arg).map_err(|_| ())?);
            Ok(vec)
        })?))
    }
}

impl<T> From<VecEncoding<T>> for Multipart where T: MessageEncode<T> {
    fn from(vec: VecEncoding<T>) -> Self {
        vec.iter().map(Message::from).collect()
    }
}
