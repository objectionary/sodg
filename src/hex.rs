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

use crate::Hex;
use anyhow::{Context, Result};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

impl Debug for Hex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.print().as_str())
    }
}

impl PartialEq for Hex {
    fn eq(&self, other: &Self) -> bool {
        self.bytes() == other.bytes()
    }
}

impl Eq for Hex {}

impl Display for Hex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.print().as_str())
    }
}

impl Hex {
    /// Empty Hex, for performance improvement.
    const BLANK: [u8; 24] = [0_u8; 24];

    /// Make an empty `Hex`.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::empty();
    /// assert!(d.is_empty());
    /// assert_eq!("--", d.print());
    /// ```
    #[must_use]
    #[inline]
    pub const fn empty() -> Self {
        Self::Bytes(Self::BLANK, 0)
    }

    /// Take the bytes contained.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from(2);
    /// assert_eq!(8, d.len())
    /// ```
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        match self {
            Self::Vector(v) => v,
            Self::Bytes(array, size) => &array[..*size],
        }
    }

    /// Count, how many bytes are in there.
    ///
    /// For example, an empty [`Hex`] has zero bytes:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::empty();
    /// assert_eq!(0, d.len());
    /// ```
    ///
    /// A non-empty [`Hex`] with an `i64` inside has eight bytes:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from(42);
    /// assert_eq!(8, d.len());
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Vector(x) => x.len(),
            Self::Bytes(_, size) => *size,
        }
    }

    /// Create a new [`Hex`] from slice, in appropriate mode.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_slice(&[0xDE, 0xAD]);
    /// assert_eq!("DE-AD", d.print());
    /// let v = Hex::from_slice(&vec![0xBE, 0xEF]);
    /// assert_eq!("BE-EF", v.print());
    /// ```
    #[must_use]
    pub fn from_slice(slice: &[u8]) -> Self {
        if slice.len() <= 24 {
            Self::Bytes(
                {
                    let mut x = [0; 24];
                    x[..slice.len()].copy_from_slice(slice);
                    x
                },
                slice.len(),
            )
        } else {
            Self::Vector(slice.to_vec())
        }
    }

    /// Create a new [`Hex`] from `Vec<u8>`.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_vec(vec![0xCA, 0xFE]);
    /// assert_eq!("CA-FE", d.print());
    /// ```
    #[must_use]
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        if bytes.len() <= 24 {
            Self::from_slice(&bytes)
        } else {
            Self::Vector(bytes)
        }
    }

    /// Create a new [`Hex`] from the bytes composing `&str`.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_str_bytes("Ура!");
    /// assert_eq!("D0-A3-D1-80-D0-B0-21", d.print());
    /// ```
    #[must_use]
    pub fn from_str_bytes(d: &str) -> Self {
        Self::from_slice(d.as_bytes())
    }

    /// Is it empty and has no data (not a single byte)?
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_vec(vec![]);
    /// assert_eq!(true, d.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Turn it into `bool`.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_vec([0x01].to_vec());
    /// assert_eq!(true, d.to_bool());
    /// ```
    #[must_use]
    pub fn to_bool(&self) -> bool {
        self.bytes()[0] == 0x01
    }

    /// Turn it into `i64`.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_vec([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2A].to_vec());
    /// assert_eq!(42, d.to_i64().unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// If it's impossible to convert to an integer, an error will be returned.
    pub fn to_i64(&self) -> Result<i64> {
        let a: &[u8; 8] = &self.bytes().try_into().with_context(|| {
            format!(
                "There is not enough bytes, can't make INT (just {} while we need eight)",
                self.bytes().len()
            )
        })?;
        Ok(i64::from_be_bytes(*a))
    }

    /// Turn it into `f64`.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_vec([0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18].to_vec());
    /// assert_eq!(std::f64::consts::PI, d.to_f64().unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// If it's impossible to convert to a float, an error will be returned.
    pub fn to_f64(&self) -> Result<f64> {
        let a: &[u8; 8] = &self.bytes().try_into().with_context(|| {
            format!(
                "There is not enough bytes, can't make FLOAT (just {} while we need eight)",
                self.bytes().len()
            )
        })?;
        Ok(f64::from_be_bytes(*a))
    }

    /// Turn it into `String` in UTF-8 encoding.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_vec([0x41, 0x42].to_vec());
    /// assert_eq!("AB", d.to_utf8().unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// If it's impossible to convert to a UTF-8 string, an error will be returned.
    pub fn to_utf8(&self) -> Result<String> {
        String::from_utf8(self.bytes().to_vec())
            .with_context(|| format!("The string inside Hex is not UTF-8 ({} bytes)", self.len()))
    }

    /// Turn it into a hexadecimal string.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_vec([0xCA, 0xFE].to_vec());
    /// assert_eq!("CA-FE", d.print());
    /// ```
    ///
    /// A string of one letter will be printed as `xx`, without the trailing dash:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_vec([0xCA].to_vec());
    /// assert_eq!("CA", d.print());
    /// ```
    ///
    /// An empty string will be printed as `--`:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::empty();
    /// assert_eq!("--", d.print());
    /// ```
    #[must_use]
    pub fn print(&self) -> String {
        if self.bytes().is_empty() {
            "--".to_string()
        } else {
            self.bytes()
                .iter()
                .map(|b| format!("{b:02X}"))
                .collect::<Vec<String>>()
                .join("-")
        }
    }

    /// Turn it into a vector of bytes (making a clone).
    #[must_use]
    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes().to_vec()
    }

    /// Take one byte.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_str_bytes("你好");
    /// assert_eq!("E4-BD-A0-E5-A5-BD", d.print());
    /// assert_eq!(0xA0, d.byte_at(2));
    /// ```
    #[must_use]
    pub fn byte_at(&self, pos: usize) -> u8 {
        self.bytes()[pos]
    }

    /// Skip a few bytes at the beginning and return the rest
    /// as a new instance of `Hex`.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from_str_bytes("Hello, world!");
    /// assert_eq!("world!", d.tail(7).to_utf8().unwrap());
    /// ```
    #[must_use]
    pub fn tail(&self, skip: usize) -> Self {
        Self::from_vec(self.bytes()[skip..].to_vec())
    }

    /// Create a new `Hex`, which is a concatenation of `self` and `h`.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// let a = Hex::from_slice("dead".as_bytes());
    /// let b = Hex::from_slice("beef".as_bytes());
    /// let c = a.concat(&b);
    /// assert_eq!(c, Hex::from_slice("deadbeef".as_bytes()));
    /// ```
    #[must_use]
    pub fn concat(&self, h: &Self) -> Self {
        match &self {
            Self::Vector(v) => {
                let mut vx = v.clone();
                vx.extend_from_slice(h.bytes());
                Self::Vector(vx)
            }
            Self::Bytes(b, l) => {
                if l + h.len() <= 24 {
                    let mut bytes = *b;
                    bytes[*l..*l + h.len()].copy_from_slice(h.bytes());
                    Self::Bytes(bytes, l + h.len())
                } else {
                    let mut v = Vec::new();
                    v.extend_from_slice(b);
                    v.extend_from_slice(h.bytes());
                    Self::Vector(v)
                }
            }
        }
    }
}

impl From<i64> for Hex {
    /// Make a new `Hex` from `i64`.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from(65536);
    /// assert_eq!("00-00-00-00-00-01-00-00", d.print());
    /// ```
    fn from(d: i64) -> Self {
        Self::from_slice(&d.to_be_bytes())
    }
}

impl From<f64> for Hex {
    /// Make a new `Hex` from `f64`.
    ///
    /// For example:
    ///
    /// ```
    /// use std::f64::consts::PI;
    /// use sodg::Hex;
    /// let d = Hex::from(PI);
    /// assert_eq!("40-09-21-FB-54-44-2D-18", d.print());
    /// ```
    fn from(d: f64) -> Self {
        Self::from_slice(&d.to_be_bytes())
    }
}

impl From<bool> for Hex {
    /// Create a new [`Hex`] from `bool`.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// let d = Hex::from(true);
    /// assert_eq!("01", d.print());
    /// ```
    fn from(d: bool) -> Self {
        Self::from_slice(&(if d { [1] } else { [0] }))
    }
}

impl FromStr for Hex {
    type Err = anyhow::Error;

    /// Create a `Hex` from a `&str` containing a hexadecimal representation of data.
    ///
    /// For example, this is how you make a new [`Hex`] from `"DE-AD-BE-EF-20-22"` string:
    ///
    /// ```
    /// use sodg::Hex;
    /// use std::str::FromStr;
    /// let hex = "DE-AD-BE-EF-20-22";
    /// let d: Hex = hex.parse().unwrap();
    /// let d2 = Hex::from_str(hex).unwrap();
    /// assert_eq!("DE-AD-BE-EF-20-22", d.print());
    /// assert_eq!("DE-AD-BE-EF-20-22", d2.print());
    /// ```
    ///
    /// An empty `Hex` may be created either from an empty string
    /// or `"--"`:
    ///
    /// ```
    /// use sodg::Hex;
    /// use std::str::FromStr;
    /// let d1: Hex = Hex::from_str("--").unwrap();
    /// let d2: Hex = Hex::from_str("").unwrap();
    /// assert_eq!(Hex::empty(), d1);
    /// assert_eq!(Hex::empty(), d2);
    /// ```
    ///
    /// # Errors
    ///
    /// If it's impossible to convert from a String, an error will be returned.
    fn from_str(hex: &str) -> std::result::Result<Self, Self::Err> {
        let s = hex.replace('-', "");
        Ok(Self::from_vec(hex::decode(s)?))
    }
}

#[test]
fn simple_int() {
    let i = 42;
    let d = Hex::from(i);
    assert_eq!(i, d.to_i64().unwrap());
    assert_eq!("00-00-00-00-00-00-00-2A", d.print());
}

#[test]
fn simple_bool() {
    let b = true;
    let d = Hex::from(b);
    assert_eq!(b, d.to_bool());
    assert_eq!("01", d.print());
}

#[test]
fn simple_float() {
    let f = std::f64::consts::PI;
    let d = Hex::from(f);
    assert_eq!(f, d.to_f64().unwrap());
    assert_eq!("40-09-21-FB-54-44-2D-18", d.print());
}

#[test]
fn compares_with_data() {
    let i = 42;
    let left = Hex::from(i);
    let right = Hex::from(i);
    assert_eq!(left, right);
}

#[test]
fn prints_bytes() {
    let txt = "привет";
    let d = Hex::from_str_bytes(txt);
    assert_eq!("D0-BF-D1-80-D0-B8-D0-B2-D0-B5-D1-82", d.print());
    assert_eq!(txt, Hex::from_str(&d.print()).unwrap().to_utf8().unwrap());
}

#[test]
fn prints_empty_bytes() {
    let txt = "";
    let d = Hex::from_str_bytes(txt);
    assert_eq!("--", d.print());
}

#[test]
fn broken_int_from_small_data() {
    let d = Hex::from_vec([0x01, 0x02].to_vec());
    let ret = d.to_i64();
    assert!(ret.is_err());
}

#[test]
fn broken_float_from_small_data() {
    let d = Hex::from_vec([0x00].to_vec());
    let ret = d.to_f64();
    assert!(ret.is_err());
}

#[test]
fn direct_access_to_vec() {
    let d = Hex::from_vec([0x1F, 0x01].to_vec());
    assert_eq!(0x1F, *d.bytes().first().unwrap());
}

#[test]
fn not_enough_data_for_int() {
    let d = Hex::from_vec(vec![0x00, 0x2A]);
    assert!(d.to_i64().is_err());
}

#[test]
fn not_enough_data_for_float() {
    let d = Hex::from_vec(vec![0x00, 0x2A]);
    assert!(d.to_f64().is_err());
}

#[test]
fn too_much_data_for_int() {
    let d = Hex::from_vec(vec![0x00, 0x2A, 0x00, 0x2A, 0x00, 0x2A, 0x00, 0x2A, 0x11]);
    assert!(d.to_i64().is_err());
}

#[test]
fn makes_string() {
    let d = Hex::from_vec(vec![0x41, 0x42, 0x43]);
    assert_eq!("ABC", d.to_utf8().unwrap().as_str());
}

#[test]
fn empty_string() {
    let d = Hex::from_vec(vec![]);
    assert_eq!("", d.to_utf8().unwrap().as_str());
}

#[test]
fn non_utf8_string() {
    let d = Hex::from_vec(vec![0x00, 0xEF]);
    assert!(d.to_utf8().is_err());
}

#[test]
fn takes_tail() {
    let d = Hex::from_str_bytes("Hello, world!");
    assert_eq!("world!", d.tail(7).to_utf8().unwrap());
}

#[test]
fn takes_one_byte() {
    let d = Hex::from_str_bytes("Ура!");
    assert_eq!("D0-A3-D1-80-D0-B0-21", d.print());
    assert_eq!(0xD1, d.byte_at(2));
}

#[test]
fn measures_length() {
    let d = Hex::from_str_bytes("Ура!");
    assert_eq!(7, d.len());
}

#[test]
fn correct_equality() {
    let d = Hex::from_str("DE-AD-BE-EF").unwrap();
    let d1 = Hex::from_str("AA-BB").unwrap();
    let d2 = Hex::from_str("DE-AD-BE-EF").unwrap();
    assert_eq!(d, d);
    assert_ne!(d, d1);
    assert_eq!(d, d2);
}

#[test]
fn concat_test() {
    let a = Hex::from_str("DE-AD").unwrap();
    let b = Hex::from_str("BE-EF").unwrap();
    assert_eq!(a.concat(&b), Hex::from_str("DE-AD-BE-EF").unwrap());
}

#[test]
fn creates_from_big_slice() {
    let s: [u8; 9] = [0xAB, 0xD8, 0xAB, 0xD8, 0xAB, 0xD8, 0xAB, 0xD8, 0xAB];
    let mut accum = vec![];
    for el in s {
        accum.push(el);
        accum.push(el);
        accum.push(el);
    }
    let h = Hex::from_slice(accum.as_slice());
    assert_eq!(27, h.len());
    assert_eq!(h.to_vec(), accum);
}

#[test]
fn concatenates_from_hex_vec() {
    let a = Hex::from_vec(vec![0x12, 0xAB]);
    let b = Hex::from_slice(b"as_bytesss");
    let c = Hex::from_vec(vec![0x12, 0xAD]);
    let res = a.concat(&b).concat(&c);
    assert_eq!(14, res.len());
}

#[test]
fn concatenates_from_hex_str() {
    let a = Hex::from_str_bytes("Привет!");
    let b = Hex::from_vec(vec![0x01, 0x02]);
    let c = Hex::from_str_bytes("Пока!");
    let res = a.concat(&b).concat(&c);
    assert_eq!(24, res.len());
}
