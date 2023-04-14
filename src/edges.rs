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

use crate::{Edges, EdgesIntoIter, Label, Roll, RollIntoIter, ROLL_LIMIT};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

impl<'a, K: Clone, V: Clone> Iterator for RollIntoIter<'a, K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < ROLL_LIMIT {
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

impl<'a, K: Copy + PartialEq, V: Copy> IntoIterator for &'a Roll<K, V> {
    type Item = (K, V);
    type IntoIter = RollIntoIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        RollIntoIter {
            pos: 0,
            items: &self.items,
        }
    }
}

impl<K: Copy + PartialEq, V: Copy> Roll<K, V> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            items: [None; ROLL_LIMIT],
        }
    }

    /// Make an iterator over all pairs.
    pub const fn into_iter(&self) -> RollIntoIter<K, V> {
        RollIntoIter {
            pos: 0,
            items: &self.items,
        }
    }

    /// Is it empty?
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the total number of pairs inside.
    pub fn len(&self) -> usize {
        let mut busy = 0;
        for i in 0..ROLL_LIMIT {
            if self.items[i].is_some() {
                busy += 1;
            }
        }
        busy
    }

    /// Insert a single pair into it.
    ///
    /// # Panics
    ///
    /// It may panic if you attempt to insert too many pairs.
    pub fn insert(&mut self, k: K, v: V) {
        for i in 0..ROLL_LIMIT {
            if let Some((bk, _bv)) = self.items[i] {
                if bk == k {
                    self.items[i] = None;
                    break;
                }
            }
        }
        for i in 0..ROLL_LIMIT {
            if self.items[i].is_none() {
                self.items[i] = Some((k, v));
                return;
            }
        }
        panic!("Out of space!")
    }
}

impl Serialize for Edges {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.map.len()))?;
        for (a, v) in self {
            map.serialize_entry(&a, &v)?;
        }
        map.end()
    }
}

struct Vi;

impl<'de> Visitor<'de> for Vi {
    type Value = Edges;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a map of edges")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut edges = Edges::new();
        while let Some((key, value)) = access.next_entry()? {
            edges.insert(key, value);
        }
        Ok(edges)
    }
}

impl<'de> Deserialize<'de> for Edges {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(Vi)
    }
}

impl<'a> Iterator for EdgesIntoIter<'a> {
    type Item = (Label, u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> IntoIterator for &'a Edges {
    type Item = (Label, u32);
    type IntoIter = EdgesIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        EdgesIntoIter {
            iter: self.map.into_iter(),
        }
    }
}

impl Edges {
    pub const fn new() -> Self {
        Self { map: Roll::new() }
    }

    pub fn insert(&mut self, a: Label, v: u32) {
        self.map.insert(a, v);
    }
}

#[cfg(test)]
use anyhow::Result;

#[cfg(test)]
use bincode::{deserialize, serialize};

#[test]
fn serialize_and_deserialize() -> Result<()> {
    let mut before = Edges::new();
    before.insert(Label::Alpha(0), 42);
    let bytes: Vec<u8> = serialize(&before)?;
    let after: Edges = deserialize(&bytes)?;
    assert_eq!(42, after.into_iter().next().unwrap().1);
    Ok(())
}

#[test]
fn insert_and_check_length() -> Result<()> {
    let mut roll = Roll::new();
    roll.insert(Label::Alpha(0), 42);
    assert_eq!(1, roll.len());
    roll.insert(Label::Alpha(1), 16);
    assert_eq!(2, roll.len());
    roll.insert(Label::Alpha(0), 16);
    assert_eq!(2, roll.len());
    Ok(())
}

#[test]
fn empty_length() -> Result<()> {
    let roll: Roll<u32, u32> = Roll::new();
    assert_eq!(0, roll.len());
    Ok(())
}

#[test]
fn empty_iterator() -> Result<()> {
    let roll: Roll<u32, u32> = Roll::new();
    assert!(roll.into_iter().next().is_none());
    Ok(())
}

#[test]
fn insert_and_jump_over_next() -> Result<()> {
    let mut roll = Roll::new();
    roll.insert(Label::Alpha(0), 42);
    let mut iter = roll.into_iter();
    assert_eq!(42, iter.next().unwrap().1);
    assert!(iter.next().is_none());
    Ok(())
}

#[test]
fn insert_and_iterate() -> Result<()> {
    let mut roll = Roll::new();
    roll.insert(Label::Alpha(0), 42);
    roll.insert(Label::Alpha(1), 16);
    let mut sum = 0;
    for (_k, v) in roll.into_iter() {
        sum += v;
    }
    assert_eq!(58, sum);
    Ok(())
}

#[test]
fn insert_and_into_iterate() -> Result<()> {
    let mut roll = Roll::new();
    roll.insert(Label::Alpha(0), 42);
    roll.insert(Label::Alpha(1), 16);
    let mut sum = 0;
    for (_k, v) in roll.into_iter() {
        sum += v;
    }
    assert_eq!(58, sum);
    Ok(())
}
