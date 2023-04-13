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

use crate::{Edges, EdgesIter, Label};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

impl Serialize for Edges {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.map.len()))?;
        for (a, v) in self.iter() {
            map.serialize_entry(a, v)?;
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

impl<'a> Iterator for EdgesIter<'a> {
    type Item = (&'a Label, &'a u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl Edges {
    pub fn new() -> Self {
        Self {
            map: hashbrown::HashMap::new(),
        }
    }

    pub fn iter(&self) -> EdgesIter {
        EdgesIter {
            iter: self.map.iter(),
        }
    }

    pub fn insert(&mut self, a: Label, v: u32) {
        self.map.insert(a, v);
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Label, &mut u32) -> bool,
    {
        self.map.retain(f);
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
    assert_eq!(42, *after.iter().next().unwrap().1);
    Ok(())
}
