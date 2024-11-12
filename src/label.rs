// Copyright (c) 2022-2024 Objectionary.com
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

use crate::Label;
use anyhow::anyhow;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

impl FromStr for Label {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with('α') {
            let tail: String = s.chars().skip(1).collect::<Vec<_>>().into_iter().collect();
            Self::Alpha(tail.parse::<usize>()?)
        } else if s.len() == 1 {
            Self::Greek(s.chars().next().unwrap())
        } else {
            let v: Vec<char> = s.chars().collect();
            let mut a: [char; 8] = [' '; 8];
            for (i, c) in v.into_iter().enumerate() {
                if i > 7 {
                    return Err(anyhow!("Can't parse more than {} chars", a.len()));
                }
                a[i] = c;
            }
            Self::Str(a)
        })
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <&Self as Debug>::fmt(&self, f)
    }
}

impl Debug for Label {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Self::Greek(c) => f.write_str(format!("{c}").as_str()),
            Self::Alpha(i) => f.write_str(format!("α{i}").as_str()),
            Self::Str(a) => {
                f.write_str(a.iter().filter(|c| **c != ' ').collect::<String>().as_str())
            }
        }
    }
}

use rstest::rstest;

#[rstest]
#[case("𝜑")]
#[case("α5")]
#[case("hello")]
fn parses_and_prints(#[case] txt: &str) {
    let l = Label::from_str(txt).unwrap();
    assert_eq!(txt, l.to_string());
}
