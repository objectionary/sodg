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

use crate::Hex;
use anyhow::{Context, Result};
use std::fmt::{Debug, Display, Formatter};

impl Clone for Hex {
    fn clone(&self) -> Self {
        Hex::from_vec(self.bytes.clone())
    }
}

impl Debug for Hex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.print().as_str())
    }
}

impl PartialEq for Hex {
    fn eq(&self, other: &Self) -> bool {
        self.print() == other.print()
    }
}

impl Display for Hex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.print().as_str())
    }
}

impl Hex {
    /// Make an empty `Hex`.
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::empty();
    /// assert!(d.is_empty());
    /// assert_eq!("--", d.print());
    /// ```
    pub fn empty() -> Self {
        Self::from_vec(Vec::new())
    }

    /// From `Vec<u8>`.
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_vec(vec![0xCA, 0xFE]);
    /// assert_eq!("CA-FE", d.print());
    /// ```
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Hex { bytes }
    }

    /// From `String` as HEX, for example `DE-AD-BE-EF-20-22`.
    ///
    /// ```
    /// use sodg::Hex;
    /// let hex = "DE-AD-BE-EF-20-22";
    /// let d = Hex::parse(hex.to_string());
    /// assert_eq!("DE-AD-BE-EF-20-22", d.print());
    /// ```
    pub fn parse(hex: String) -> Self {
        let s = hex.replace('-', "");
        Self::from_vec(hex::decode(s).unwrap())
    }

    /// Make `Hex` from `i64`.
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_i64(65536);
    /// assert_eq!("00-00-00-00-00-01-00-00", d.print());
    /// ```
    pub fn from_i64(d: i64) -> Self {
        Self::from_vec(d.to_be_bytes().to_vec())
    }

    /// From `bool`.
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_bool(true);
    /// assert_eq!("01", d.print());
    /// ```
    pub fn from_bool(d: bool) -> Self {
        Self::from_vec(if d { [1] } else { [0] }.to_vec())
    }

    /// Make `Hex` from `f64`.
    ///
    /// ```
    /// use std::f64::consts::PI;
    /// use sodg::Hex;
    /// let d = Hex::from_f64(PI);
    /// assert_eq!("40-09-21-FB-54-44-2D-18", d.print());
    /// ```
    pub fn from_f64(d: f64) -> Self {
        Self::from_vec(d.to_be_bytes().to_vec())
    }

    /// Make `Hex` from `String`.
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_string("Ура!".to_string());
    /// assert_eq!("D0-A3-D1-80-D0-B0-21", d.print());
    /// ```
    pub fn from_string(d: String) -> Self {
        Self::from_vec(d.as_bytes().to_vec())
    }

    /// From `&str`.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(d: &str) -> Self {
        Self::from_vec(d.to_string().as_bytes().to_vec())
    }

    /// It's empty and no data?
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_vec(vec![]);
    /// assert_eq!(true, d.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.bytes.len() == 0
    }

    /// Turn it into `bool`.
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_vec([0x01].to_vec());
    /// assert_eq!(true, d.to_bool().unwrap());
    /// ```
    pub fn to_bool(&self) -> Result<bool> {
        Ok(self.bytes[0] == 0x01)
    }

    /// Turn it into `i64`.
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_vec([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2A].to_vec());
    /// assert_eq!(42, d.to_i64().unwrap());
    /// ```
    pub fn to_i64(&self) -> Result<i64> {
        let a: &[u8; 8] = &self
            .bytes
            .as_slice()
            .try_into()
            .context("There is not enough data, can't make INT")?;
        Ok(i64::from_be_bytes(*a))
    }

    /// Turn it into `f64`.
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_vec([0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18].to_vec());
    /// assert_eq!(std::f64::consts::PI, d.to_f64().unwrap());
    /// ```
    pub fn to_f64(&self) -> Result<f64> {
        let a: &[u8; 8] = &self
            .bytes
            .as_slice()
            .try_into()
            .context("There is no data, can't make FLOAT")?;
        Ok(f64::from_be_bytes(*a))
    }

    /// Turn it into `string`.
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_vec([0x41, 0x42].to_vec());
    /// assert_eq!("AB", d.to_utf8().unwrap());
    /// ```
    pub fn to_utf8(&self) -> Result<String> {
        String::from_utf8(self.bytes.clone()).context(format!(
            "The string inside Hex is not UTF-8 ({} bytes)",
            self.bytes.len()
        ))
    }

    /// Turn it into a hexadecimal string.
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_vec([0xCA, 0xFE].to_vec());
    /// assert_eq!("CA-FE", d.print());
    /// ```
    pub fn print(&self) -> String {
        if self.bytes.is_empty() {
            "--".to_string()
        } else {
            self.bytes
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<String>>()
                .join("-")
        }
    }

    /// Turn it into a vector of bytes (making a clone).
    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    /// Return a reference to the encapsulated immutable vec.
    pub fn as_vec(&self) -> &Vec<u8> {
        &self.bytes
    }

    /// Take one byte.
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_str("你好");
    /// assert_eq!("E4-BD-A0-E5-A5-BD", d.print());
    /// assert_eq!(0xA0, d.byte_at(2));
    /// ```
    pub fn byte_at(&self, pos: usize) -> u8 {
        self.bytes[pos]
    }

    /// Skip a few bytes at the beginning and return the rest
    /// as a new instance of `Hex`.
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_str("Hello, world!");
    /// assert_eq!("world!", d.tail(7).to_utf8().unwrap());
    /// ```
    pub fn tail(&self, skip: usize) -> Self {
        Self::from_vec(self.bytes[skip..].to_vec())
    }
}

#[test]
fn simple_int() -> Result<()> {
    let i = 42;
    let d = Hex::from_i64(i);
    assert_eq!(i, d.to_i64()?);
    assert_eq!("00-00-00-00-00-00-00-2A", d.print());
    Ok(())
}

#[test]
fn simple_bool() -> Result<()> {
    let b = true;
    let d = Hex::from_bool(b);
    assert_eq!(b, d.to_bool()?);
    assert_eq!("01", d.print());
    Ok(())
}

#[test]
fn simple_float() -> Result<()> {
    let f = std::f64::consts::PI;
    let d = Hex::from_f64(f);
    assert_eq!(f, d.to_f64()?);
    assert_eq!("40-09-21-FB-54-44-2D-18", d.print());
    Ok(())
}

#[test]
fn compares_with_data() -> Result<()> {
    let i = 42;
    let left = Hex::from_i64(i);
    let right = Hex::from_i64(i);
    assert_eq!(left, right);
    Ok(())
}

#[test]
fn prints_bytes() -> Result<()> {
    let txt = "привет";
    let d = Hex::from_str(txt);
    assert_eq!("D0-BF-D1-80-D0-B8-D0-B2-D0-B5-D1-82", d.print());
    assert_eq!(txt, Hex::parse(d.print()).to_utf8()?);
    Ok(())
}

#[test]
fn prints_empty_bytes() -> Result<()> {
    let txt = "";
    let d = Hex::from_str(txt);
    assert_eq!("--", d.print());
    Ok(())
}

#[test]
fn broken_int_from_small_data() -> Result<()> {
    let d = Hex::from_vec([0x01, 0x02].to_vec());
    let ret = d.to_i64();
    assert!(ret.is_err());
    Ok(())
}

#[test]
fn broken_float_from_small_data() -> Result<()> {
    let d = Hex::from_vec([0x00].to_vec());
    let ret = d.to_f64();
    assert!(ret.is_err());
    Ok(())
}

#[test]
fn direct_access_to_vec() -> Result<()> {
    let d = Hex::from_vec([0x1F, 0x01].to_vec());
    assert_eq!(0x1F, *d.as_vec().get(0).unwrap());
    Ok(())
}

#[test]
fn not_enough_data_for_int() -> Result<()> {
    let d = Hex::from_vec(vec![0x00, 0x2A]);
    assert!(d.to_i64().is_err());
    Ok(())
}

#[test]
fn not_enough_data_for_float() -> Result<()> {
    let d = Hex::from_vec(vec![0x00, 0x2A]);
    assert!(d.to_f64().is_err());
    Ok(())
}

#[test]
fn too_much_data_for_int() -> Result<()> {
    let d = Hex::from_vec(vec![0x00, 0x2A, 0x00, 0x2A, 0x00, 0x2A, 0x00, 0x2A, 0x11]);
    assert!(d.to_i64().is_err());
    Ok(())
}

#[test]
fn makes_string() -> Result<()> {
    let d = Hex::from_vec(vec![0x41, 0x42, 0x43]);
    assert_eq!("ABC", d.to_utf8()?.as_str());
    Ok(())
}

#[test]
fn empty_string() -> Result<()> {
    let d = Hex::from_vec(vec![]);
    assert_eq!("", d.to_utf8()?.as_str());
    Ok(())
}

#[test]
fn non_utf8_string() -> Result<()> {
    let d = Hex::from_vec(vec![0x00, 0xEF]);
    assert!(d.to_utf8().is_err());
    Ok(())
}

#[test]
fn takes_tail() -> Result<()> {
    let d = Hex::from_str("Hello, world!");
    assert_eq!("world!", d.tail(7).to_utf8()?);
    Ok(())
}

#[test]
fn takes_one_byte() -> Result<()> {
    let d = Hex::from_str("Ура!");
    assert_eq!("D0-A3-D1-80-D0-B0-21", d.print());
    assert_eq!(0xD1, d.byte_at(2));
    Ok(())
}
