// Copyright (c) 2022-2023 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::fmt::Formatter;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use crate::Pile;

impl<V : Clone, const N: usize> Clone for Pile<V, N> {
    /// Clone it.
    fn clone(&self) -> Self {
        let mut p: Self = Self::new();
        for v in self.iter() {
            p.push(v.clone());
        }
        p
    }
}

impl<V : Clone, const N: usize> Pile<V, N> {
    /// Make it.
    #[inline]
    pub const fn new() -> Self {
        unsafe {
            Self {
                next: 0,
                items: MaybeUninit::<[MaybeUninit<V>; N]>::uninit().assume_init(),
            }
        }
    }

    /// Make it from vector.
    #[inline]
    pub fn from_vec(v: Vec<V>) -> Self {
        let mut p = Self::new();
        for i in v {
            p.push(i);
        }
        p
    }

    /// Push new element into it.
    #[inline]
    pub fn push(&mut self, v: V) {
        self.items[self.next].write(v);
        self.next += 1;
    }

    /// Clear.
    #[inline]
    pub fn clear(&mut self) {
        self.next = 0;
    }

    /// Is it empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Length of it.
    #[inline]
    pub const fn len(&self) -> usize {
        self.next
    }

    /// Iterate them.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item=&V> {
        self.items.iter().take(self.next).map(|mu| unsafe { mu.assume_init_ref() })
    }
}

impl<V: Serialize + Clone, const N: usize> Serialize for Pile<V, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut map = serializer.serialize_seq(Some(self.next))?;
        for v in self.iter() {
            map.serialize_element(v)?;
        }
        map.end()
    }
}

struct Vi<V, const N: usize>(PhantomData<V>);

impl<'de, V: Clone + Deserialize<'de>, const N: usize> Visitor<'de> for Vi<V, N> {
    type Value = Pile<V, N>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a Pile")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
    {
        let mut p: Self::Value = Pile::new();
        while let Some(v) = access.next_element()? {
            p.push(v);
        }
        Ok(p)
    }
}

impl<'de, V: Clone + Deserialize<'de>, const N: usize> Deserialize<'de>
for Pile<V, N>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(Vi(PhantomData))
    }
}

#[cfg(test)]
use bincode::{deserialize, serialize};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;

#[test]
fn serialize_and_deserialize() {
    let mut before: Pile<u8, 8> = Pile::new();
    before.push(42);
    let bytes: Vec<u8> = serialize(&before).unwrap();
    let after: Pile<u8, 8> = deserialize(&bytes).unwrap();
    assert_eq!(42, *after.iter().next().unwrap());
}

#[test]
fn empty_pile_serde() {
    let before: Pile<u8, 8> = Pile::new();
    let bytes: Vec<u8> = serialize(&before).unwrap();
    let after: Pile<u8, 8> = deserialize(&bytes).unwrap();
    assert!(after.is_empty());
}

#[test]
fn push_and_iterate() {
    let mut p : Pile<u64, 16> = Pile::new();
    p.push(1);
    p.push(2);
    p.push(3);
    let mut sum = 0;
    for x in p.iter() {
        sum += x;
    }
    assert_eq!(6, sum);
}

#[test]
fn push_clear_and_iterate() {
    let mut p : Pile<u64, 16> = Pile::new();
    p.push(1);
    p.push(2);
    p.push(3);
    assert_eq!(3, p.len());
    p.clear();
    assert_eq!(0, p.len());
    let mut sum = 0;
    for x in p.iter() {
        sum += x;
    }
    assert_eq!(0, sum);
}
