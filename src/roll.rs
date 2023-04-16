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

use crate::RollItem::{Absent, Present};
use crate::{Roll, RollIntoIter, RollItem, RollIter};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use std::marker::PhantomData;

impl<K, V> Default for RollItem<K, V> {
    fn default() -> Self {
        Absent
    }
}

impl<K, V> RollItem<K, V> {
    const fn is_some(&self) -> bool {
        match self {
            Absent => false,
            Present(_) => true,
        }
    }

    fn unwrap(self) -> (K, V) {
        match self {
            Present(p) => (p.0, p.1),
            Absent => panic!("Oops"),
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut (K, V)> {
        match *self {
            Present(ref mut x) => Some(x),
            Absent => None,
        }
    }
}

impl<'a, K: Clone, V: Clone, const N: usize> Iterator for RollIter<'a, K, V, N> {
    type Item = (&'a K, &'a V);

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < N {
            if let Present(p) = &self.items[self.pos] {
                self.pos += 1;
                return Some((&p.0, &p.1));
            }
            self.pos += 1;
        }
        None
    }
}

impl<'a, K: Clone, V: Clone, const N: usize> Iterator for RollIntoIter<'a, K, V, N> {
    type Item = (K, V);

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < N {
            if self.items[self.pos].is_some() {
                let pair = self.items[self.pos].clone().unwrap();
                self.pos += 1;
                return Some(pair);
            }
            self.pos += 1;
        }
        None
    }
}

impl<'a, K: Copy + PartialEq, V: Clone, const N: usize> IntoIterator for &'a Roll<K, V, N> {
    type Item = (K, V);
    type IntoIter = RollIntoIter<'a, K, V, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RollIntoIter {
            pos: 0,
            items: &self.items,
        }
    }
}

impl<K: Copy + PartialEq, V: Clone, const N: usize> Default for Roll<K, V, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Copy + PartialEq, V: Clone, const N: usize> Roll<K, V, N> {
    /// Make it.
    #[must_use]
    pub fn new() -> Self {
        Self {
            items: [(); N].map(|_| RollItem::<K, V>::default()),
        }
    }

    /// Make an iterator over all pairs.
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> RollIter<K, V, N> {
        RollIter {
            pos: 0,
            items: &self.items,
        }
    }

    /// Make an iterator over all pairs.
    #[inline]
    #[must_use]
    pub const fn into_iter(&self) -> RollIntoIter<K, V, N> {
        RollIntoIter {
            pos: 0,
            items: &self.items,
        }
    }

    /// Is it empty?
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the total number of pairs inside.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        let mut busy = 0;
        for i in 0..N {
            if self.items[i].is_some() {
                busy += 1;
            }
        }
        busy
    }

    /// Contains this key?
    #[inline]
    pub fn contains_key(&self, k: K) -> bool {
        for i in 0..N {
            if let Present((bk, _bv)) = &self.items[i] {
                if *bk == k {
                    return true;
                }
            }
        }
        false
    }

    /// Remove by key.
    #[inline]
    pub fn remove(&mut self, k: K) {
        for i in 0..N {
            if let Present((bk, _bv)) = &self.items[i] {
                if *bk == k {
                    self.items[i] = Absent;
                    break;
                }
            }
        }
    }

    /// Insert a single pair into it.
    ///
    /// # Panics
    ///
    /// It may panic if you attempt to insert too many pairs.
    #[inline]
    pub fn insert(&mut self, k: K, v: V) {
        self.remove(k);
        for i in 0..N {
            if !self.items[i].is_some() {
                self.items[i] = Present((k, v));
                return;
            }
        }
        panic!("Out of space!")
    }

    /// Get a reference to a single value.
    #[inline]
    #[must_use]
    pub fn get(&self, k: K) -> Option<&V> {
        for i in 0..N {
            if let Present(p) = &self.items[i] {
                if p.0 == k {
                    return Some(&p.1);
                }
            }
        }
        None
    }

    /// Get a mutable reference to a single value.
    ///
    /// # Panics
    ///
    /// If can't turn it into a mutable state.
    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, k: K) -> Option<&mut V> {
        for i in 0..N {
            if let Present(p) = &mut self.items[i] {
                if p.0 == k {
                    return Some(&mut self.items[i].as_mut().unwrap().1);
                }
            }
        }
        None
    }
}

impl<K: Copy + PartialEq + Serialize, V: Clone + Serialize, const N: usize> Serialize
    for Roll<K, V, N>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (a, v) in self {
            map.serialize_entry(&a, &v)?;
        }
        map.end()
    }
}

struct Vi<K, V, const N: usize>(PhantomData<K>, PhantomData<V>);

impl<'de, K: Copy + PartialEq + Deserialize<'de>, V: Clone + Deserialize<'de>, const N: usize>
    Visitor<'de> for Vi<K, V, N>
{
    type Value = Roll<K, V, N>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a roll")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut roll: Self::Value = Roll::new();
        while let Some((key, value)) = access.next_entry()? {
            roll.insert(key, value);
        }
        Ok(roll)
    }
}

impl<'de, K: Copy + PartialEq + Deserialize<'de>, V: Clone + Deserialize<'de>, const N: usize>
    Deserialize<'de> for Roll<K, V, N>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(Vi(PhantomData, PhantomData))
    }
}

#[cfg(test)]
use anyhow::Result;

#[cfg(test)]
use bincode::{deserialize, serialize};

#[test]
fn insert_and_check_length() -> Result<()> {
    let mut roll: Roll<&str, i32, 10> = Roll::new();
    roll.insert("first", 42);
    assert_eq!(1, roll.len());
    roll.insert("second", 16);
    assert_eq!(2, roll.len());
    roll.insert("first", 16);
    assert_eq!(2, roll.len());
    Ok(())
}

#[test]
fn empty_length() -> Result<()> {
    let roll: Roll<u32, u32, 10> = Roll::new();
    assert_eq!(0, roll.len());
    Ok(())
}

#[test]
fn empty_iterator() -> Result<()> {
    let roll: Roll<u32, u32, 4> = Roll::new();
    assert!(roll.into_iter().next().is_none());
    Ok(())
}

#[test]
fn insert_and_jump_over_next() -> Result<()> {
    let mut roll: Roll<&str, i32, 10> = Roll::new();
    roll.insert("foo", 42);
    let mut iter = roll.into_iter();
    assert_eq!(42, iter.next().unwrap().1);
    assert!(iter.next().is_none());
    Ok(())
}

#[test]
fn insert_and_iterate() -> Result<()> {
    let mut roll: Roll<&str, i32, 10> = Roll::new();
    roll.insert("one", 42);
    roll.insert("two", 16);
    let mut sum = 0;
    for (_k, v) in roll.iter() {
        sum += v;
    }
    assert_eq!(58, sum);
    Ok(())
}

#[test]
fn insert_and_into_iterate() -> Result<()> {
    let mut roll: Roll<&str, i32, 10> = Roll::new();
    roll.insert("one", 42);
    roll.insert("two", 16);
    let mut sum = 0;
    for (_k, v) in roll.into_iter() {
        sum += v;
    }
    assert_eq!(58, sum);
    Ok(())
}

#[test]
fn insert_and_gets() -> Result<()> {
    let mut roll: Roll<&str, i32, 10> = Roll::new();
    roll.insert("one", 42);
    roll.insert("two", 16);
    assert_eq!(16, *roll.get("two").unwrap());
    Ok(())
}

#[test]
fn insert_and_gets_mut() -> Result<()> {
    let mut roll: Roll<i32, [i32; 3], 10> = Roll::new();
    roll.insert(42, [1, 2, 3]);
    let a = roll.get_mut(42).unwrap();
    a[0] = 500;
    assert_eq!(500, roll.get(42).unwrap()[0]);
    Ok(())
}

#[test]
fn serialize_and_deserialize() -> Result<()> {
    let mut before: Roll<u8, u8, 8> = Roll::new();
    before.insert(1, 42);
    let bytes: Vec<u8> = serialize(&before)?;
    let after: Roll<u8, u8, 8> = deserialize(&bytes)?;
    assert_eq!(42, after.into_iter().next().unwrap().1);
    Ok(())
}

#[cfg(test)]
#[derive(Clone)]
struct Foo {
    v: Vec<u32>,
}

#[test]
fn insert_struct() -> Result<()> {
    let mut roll: Roll<u8, Foo, 8> = Roll::new();
    let foo = Foo { v: vec![1, 2, 100] };
    roll.insert(1, foo);
    assert_eq!(100, roll.into_iter().next().unwrap().1.v[2]);
    Ok(())
}

#[cfg(test)]
#[derive(Clone)]
struct Composite {
    r: Roll<u8, u8, 1>,
}

#[test]
fn insert_composite() -> Result<()> {
    let mut roll: Roll<u8, Composite, 8> = Roll::new();
    let c = Composite { r: Roll::new() };
    roll.insert(1, c);
    assert_eq!(0, roll.into_iter().next().unwrap().1.r.len());
    Ok(())
}

#[test]
fn large_roll_in_heap() -> Result<()> {
    let roll: Box<Roll<u64, [u64; 10], 10>> = Box::new(Roll::new());
    assert_eq!(0, roll.len());
    Ok(())
}
