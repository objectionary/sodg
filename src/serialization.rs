// Copyright (c) 2022 Yegor Bugayenko
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

use crate::Sot;
use anyhow::{Context, Result};
use bincode::{deserialize, serialize};
use log::trace;
use std::fs;
use std::path::Path;
use std::time::Instant;

impl Sot {
    /// Save the entire Sot into a binary file. The entire Sot
    /// can be restored from the file. Returns the size of the file just saved.
    pub fn save(&mut self, path: &Path) -> Result<usize> {
        let start = Instant::now();
        let bytes: Vec<u8> = serialize(self).context("Failed to serialize")?;
        let size = bytes.len();
        fs::write(path, bytes).context(format!("Can't write to {}", path.display()))?;
        trace!(
            "Serialized {} bytes to {} in {:?}",
            size,
            path.display(),
            start.elapsed()
        );
        Ok(size)
    }

    /// Load the entire Sot from a binary file previously
    /// created by `save()`.
    pub fn load(path: &Path) -> Result<Sot> {
        let start = Instant::now();
        let bytes = fs::read(path).context(format!("Can't read from {}", path.display()))?;
        let size = bytes.len();
        let sot =
            deserialize(&bytes).context(format!("Can't deserialize from {}", path.display()))?;
        trace!(
            "Deserialized {} bytes from {} in {:?}",
            size,
            path.display(),
            start.elapsed()
        );
        Ok(sot)
    }
}

#[cfg(test)]
use tempfile::TempDir;

#[test]
fn saves_and_loads() -> Result<()> {
    let mut sot = Sot::empty();
    sot.add(0)?;
    sot.put(0, "hello".as_bytes().to_vec())?;
    sot.add(1)?;
    sot.bind(0, 1, "foo")?;
    sot.put(1, "foo".as_bytes().to_vec())?;
    let tmp = TempDir::new()?;
    let file = tmp.path().join("foo.sot");
    sot.save(file.as_path())?;
    let after = Sot::load(file.as_path())?;
    assert_eq!(sot.inspect("")?, after.inspect("")?);
    Ok(())
}
