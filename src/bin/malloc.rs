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

use sodg::{Hex, Label, Sodg};
use std::time::{Duration, Instant};

trait Book {
    fn price(&self) -> i64;
}

struct Prime {
    usd: i64,
}

impl Book for Prime {
    fn price(&self) -> i64 {
        self.usd
    }
}

struct Discounted {
    book: Box<dyn Book>,
}

impl Book for Discounted {
    fn price(&self) -> i64 {
        self.book.price() / 2
    }
}

pub fn on_graph(total: usize) -> (i64, Duration) {
    let mut g: Sodg<16> = Sodg::empty(total * 4);
    g.add(0);
    let mut sum = 0;
    let start = Instant::now();
    let fourty_two = Hex::from(42);
    for _ in 0..total {
        let v1 = 1;
        g.add(v1);
        let v2 = v1 + 1;
        g.add(v2);
        g.bind(v1, v2, Label::Alpha(0));
        g.bind(v2, v1, Label::Greek('ρ'));
        let v3 = v2 + 1;
        g.add(v3);
        g.bind(v2, v3, Label::Greek('Δ'));
        g.put(v3, &fourty_two);
        let v4 = v3 + 1;
        g.add(v4);
        g.bind(v4, v1, Label::Greek('φ'));
        assert!(g.kid(v4, Label::Alpha(0)).is_none());
        g.kid(v4, Label::Greek('φ')).unwrap();
        g.kid(v1, Label::Alpha(0)).unwrap();
        let k = g.kid(v2, Label::Greek('Δ')).unwrap();
        sum += g.data(k).unwrap().to_i64().unwrap() / 2;
    }
    (std::hint::black_box(sum), start.elapsed())
}

pub fn on_heap(total: usize) -> (i64, Duration) {
    let mut sum = 0;
    let start = Instant::now();
    for _ in 0..total {
        let prime = std::hint::black_box(Box::new(Prime { usd: 42 }));
        let discounted = std::hint::black_box(Box::new(Discounted { book: prime }));
        let price = discounted.price();
        sum += price;
    }
    (std::hint::black_box(sum), start.elapsed())
}

fn main() {
    let total = 1000000;
    let (s1, d1) = on_graph(total);
    println!("on_graph: {:?}", d1);
    let (s2, d2) = on_heap(total);
    println!("on_heap: {:?}", d2);
    println!("gain: {:.2}x", d2.as_nanos() as f64 / d1.as_nanos() as f64);
    println!("loss: {:.2}x", d1.as_nanos() as f64 / d2.as_nanos() as f64);
    assert_eq!(s1, s2);
}
