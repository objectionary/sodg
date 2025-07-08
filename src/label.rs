// SPDX-FileCopyrightText: Copyright (c) 2022-2025 Objectionary.com
// SPDX-License-Identifier: MIT

use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

use anyhow::bail;
use rstest::rstest;

use crate::Label;

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
                    bail!("Can't parse more than {} chars", a.len());
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
            Self::Greek(c) => f.write_str(c.to_string().as_str()),
            Self::Alpha(i) => f.write_str(format!("α{i}").as_str()),
            Self::Str(a) => {
                f.write_str(a.iter().filter(|c| **c != ' ').collect::<String>().as_str())
            }
        }
    }
}

#[rstest]
#[case("𝜑")]
#[case("α5")]
#[case("hello")]
fn parses_and_prints(#[case] txt: &str) {
    let l = Label::from_str(txt).unwrap();
    assert_eq!(txt, l.to_string());
}
