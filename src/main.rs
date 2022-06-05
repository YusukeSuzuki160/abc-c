// -*- coding:utf-8-unix -*-

use proconio::input;
use std::process;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [i64; n],
    }
	let mut b: Vec<Vec<i64>> = Vec::new();
	for _ in 0..k {
		let m = Vec::new();
		b.push(m);
	}
	for i in 0..n {
		b[i % k].push(a[i]);
	}
	for i in 0..k {
		b[i].sort_by_key(|x| -x);
	}
	let mut ans = Vec::new();
	for i in 0..n {
		ans.push(b[i % k].pop());
	}
	for i in 0..(n - 1) {
		if ans[i] > ans[i + 1] {
			println!("No");
			process::exit(0);
		}
	}
    println!("Yes");
}
