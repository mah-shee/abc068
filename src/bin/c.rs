#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        i: [(usize, usize); m],
    }
    let mut one_to_i = vec![false; n];
    let mut i_to_n = vec![false; n];
    for a in 0..m {
        if i[a].0 == 1 {
            one_to_i[i[a].1 - 1] = true;
        } else if i[a].1 == n {
            i_to_n[i[a].0 - 1] = true;
        }
    }
    for j in 0..n {
        if one_to_i[j] == true && i_to_n[j] == true {
            println!("POSSIBLE");
            return;
        }
    }
    println!("IMPOSSIBLE");
}
