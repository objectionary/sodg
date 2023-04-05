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

use crate::Sodg;
use anyhow::{Context, Result};
use bincode::{deserialize, serialize};
use log::trace;
use std::fs;
use std::path::Path;
use std::time::Instant;

impl Sodg {
    /// Save the entire [`Sodg`] into a binary file.
    ///
    /// The entire [`Sodg`] can be restored from the file.
    /// The function returns the size of the file just saved. In order
    /// to restore from the file, use [`Sodg::load`].
    ///
    /// # Errors
    ///
    /// If impossible to save, an error will be returned.
    pub fn save(&self, path: &Path) -> Result<usize> {
        let start = Instant::now();
        let bytes: Vec<u8> = serialize(self).with_context(|| "Failed to serialize")?;
        let size = bytes.len();
        fs::write(path, bytes).with_context(|| format!("Can't write to {}", path.display()))?;
        trace!(
            "Serialized {} vertices ({} bytes) to {} in {:?}",
            self.vertices.len(),
            size,
            path.display(),
            start.elapsed()
        );
        Ok(size)
    }

    /// Load the entire [`Sodg`] from a binary file previously
    /// created by [`Sodg::save`].
    ///
    /// # Errors
    ///
    /// If impossible to load, an error will be returned.
    pub fn load(path: &Path) -> Result<Self> {
        let start = Instant::now();
        let bytes =
            fs::read(path).with_context(|| format!("Can't read from {}", path.display()))?;
        let size = bytes.len();
        let sodg: Self = deserialize(&bytes)
            .with_context(|| format!("Can't deserialize from {}", path.display()))?;
        trace!(
            "Deserialized {} vertices ({} bytes) from {} in {:?}",
            sodg.vertices.len(),
            size,
            path.display(),
            start.elapsed()
        );
        Ok(sodg)
    }
}

#[cfg(test)]
use tempfile::TempDir;

#[cfg(test)]
use crate::Hex;

#[test]
fn saves_and_loads() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.put(0, &Hex::from_str_bytes("hello"))?;
    g.add(1)?;
    g.bind(0, 1, "foo")?;
    g.put(1, &Hex::from_str_bytes("foo"))?;
    let tmp = TempDir::new()?;
    let file = tmp.path().join("foo.sodg");
    g.save(file.as_path())?;
    let after = Sodg::load(file.as_path())?;
    assert_eq!(g.inspect("")?, after.inspect("")?);
    Ok(())
}
