// SPDX-FileCopyrightText: Copyright (c) 2022-2025 Objectionary.com
// SPDX-License-Identifier: MIT

use crate::Sodg;

impl<const N: usize> Sodg<N> {
    /// Get next unique ID of a vertex.
    ///
    /// This ID will never be returned by [`Sodg::next_id`] again. Also, this ID will not
    /// be equal to any of the existing IDs of vertices.
    ///
    /// # Panics
    ///
    /// May panic if not enough IDs are available.
    #[inline]
    pub fn next_id(&mut self) -> usize {
        let key = self.vertices.next_key();
        self.add(key);
        key
    }
}

#[test]
fn simple_next_id() {
    let mut g: Sodg<16> = Sodg::empty(256);
    assert_eq!(0, g.next_id());
    assert_eq!(1, g.next_id());
    assert_eq!(2, g.next_id());
}

#[test]
fn calculates_next_id() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    g.add(42);
    assert_eq!(1, g.next_id());
    assert_eq!(2, g.next_id());
}

#[test]
fn next_id_after_inject() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(1);
    assert_eq!(0, g.next_id());
    assert_eq!(2, g.next_id());
}

#[test]
fn next_id_after_sequence() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    g.add(1);
    assert_eq!(2, g.next_id());
    assert_eq!(3, g.next_id());
}

#[test]
fn next_id_after_zero() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    assert_eq!(1, g.next_id());
    assert_eq!(2, g.next_id());
}
