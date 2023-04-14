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

use sodg::{Label, Roll};
use std::time::Instant;
use tinymap::ArrayMap;

pub fn with_hashmap(total: usize) -> i64 {
    let mut sum = 0;
    for _ in 0..total {
        let mut map: ArrayMap<Label, i64, 10> = ArrayMap::new();
        map.insert(Label::Alpha(0), 1);
        map.insert(Label::Alpha(1), 2);
        map.insert(Label::Alpha(2), 2);
        sum += map.into_iter().find(|(_k, v)| *v == 1).unwrap().1
    }
    std::hint::black_box(sum)
}

pub fn with_roll(total: usize) -> i64 {
    let mut sum = 0;
    for _ in 0..total {
        let mut roll = Roll::new();
        roll.insert(Label::Alpha(0), 1);
        roll.insert(Label::Alpha(1), 2);
        roll.insert(Label::Alpha(2), 2);
        sum += roll.into_iter().find(|(_k, v)| *v == 1).unwrap().1
    }
    std::hint::black_box(sum)
}

fn main() {
    let total = 4000000;
    let start1 = Instant::now();
    let s1 = with_hashmap(total);
    let e1 = start1.elapsed();
    println!("with_hashmap: {:?}", e1);
    let start2 = Instant::now();
    let s2 = with_roll(total);
    let e2 = start2.elapsed();
    println!("with_roll: {:?}", e2);
    println!("gain: {:.2}x", e1.as_nanos() as f64 / e2.as_nanos() as f64);
    assert_eq!(s1, s2);
}
