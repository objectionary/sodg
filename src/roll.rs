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

use crate::{Roll, RollIntoIter, RollIter};

impl<'a, K: Clone, V: Clone, const N: usize> Iterator for RollIter<'a, K, V, N> {
    type Item = (&'a K, &'a V);

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < N {
            if let Some(p) = &self.items[self.pos] {
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

impl<'a, K: Copy + PartialEq, V: Copy, const N: usize> IntoIterator for &'a Roll<K, V, N> {
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

impl<K: Copy + PartialEq, V: Copy, const N: usize> Roll<K, V, N> {
    #[must_use]
    pub const fn new() -> Self {
        Self { items: [None; N] }
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

    /// Insert a single pair into it.
    ///
    /// # Panics
    ///
    /// It may panic if you attempt to insert too many pairs.
    #[inline]
    pub fn insert(&mut self, k: K, v: V) {
        for i in 0..N {
            if let Some((bk, _bv)) = self.items[i] {
                if bk == k {
                    self.items[i] = None;
                    break;
                }
            }
        }
        for i in 0..N {
            if self.items[i].is_none() {
                self.items[i] = Some((k, v));
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
            if let Some(p) = &self.items[i] {
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
            if let Some(p) = &mut self.items[i] {
                if p.0 == k {
                    return Some(&mut self.items[i].as_mut().unwrap().1);
                }
            }
        }
        None
    }
}

#[cfg(test)]
use anyhow::Result;

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
