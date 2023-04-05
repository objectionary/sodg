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

use sodg::{Hex, Sodg};
use std::time::Instant;

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

pub fn on_graph(total: usize) -> i64 {
    let mut sum = 0;
    let mut g = Sodg::empty();
    g.add(0).unwrap();
    for _ in 0..total {
        let v1 = g.next_id();
        g.add(v1).unwrap();
        let v2 = g.next_id();
        g.add(v2).unwrap();
        g.bind(v1, v2, "price").unwrap();
        let v3 = g.next_id();
        g.add(v3).unwrap();
        g.bind(v2, v3, "Δ").unwrap();
        g.put(v3, &Hex::from(42)).unwrap();
        let v4 = g.next_id();
        g.add(v4).unwrap();
        g.bind(v4, v1, "φ").unwrap();
        assert!(g.kid(v4, "price").is_none());
        g.kid(v4, "φ").unwrap();
        g.kid(v1, "price").unwrap();
        let k = g.kid(v2, "Δ").unwrap();
        sum += g.data(k).unwrap().to_i64().unwrap() / 2;
    }
    std::hint::black_box(sum)
}

pub fn on_heap(total: usize) -> i64 {
    let mut sum = 0;
    for _ in 0..total {
        let prime = std::hint::black_box(Box::new(Prime { usd: 42 }));
        let discounted = std::hint::black_box(Box::new(Discounted { book: prime }));
        let price = std::hint::black_box(discounted.price());
        sum += price;
    }
    std::hint::black_box(sum)
}

fn main() {
    let total = 10000;
    let start1 = Instant::now();
    let s1 = on_graph(total);
    let e1 = start1.elapsed();
    println!("on_graph: {:?}", e1);
    let start2 = Instant::now();
    let s2 = on_heap(total);
    let e2 = start2.elapsed();
    println!("on_heap: {:?}", e2);
    println!("gain: {}x", e2.as_nanos() / e1.as_nanos());
    println!("loss: {}x", e1.as_nanos() / e2.as_nanos());
    assert_eq!(s1, s2);
}
