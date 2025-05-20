// SPDX-FileCopyrightText: Copyright (c) 2022-2025 Objectionary.com
// SPDX-License-Identifier: MIT

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
    let mut g: Sodg<16> = Sodg::empty(total * 4 + 100);
    g.add(0);
    let mut sum = 0;
    let start = Instant::now();
    let fourty_two = Hex::from(42_i64);
    for i in 0..total {
        let v1 = if i % 80 == 0 { g.next_id() } else { 1 };
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
    let sizes = [
        10_000, 30_000, 60_000, 90_000, 120_000, 150_000, 180_000, 210_000, 240_000,
    ];
    for size in sizes {
        let (s1, d1) = on_graph(size);
        println!("on_graph: {:?}", d1);
        let (s2, d2) = on_heap(size);
        println!("on_heap: {:?}", d2);
        println!("gain: {:.2}x", d2.as_nanos() as f64 / d1.as_nanos() as f64);
        println!("loss: {:.2}x", d1.as_nanos() as f64 / d2.as_nanos() as f64);
        assert_eq!(s1, s2);
    }
}
