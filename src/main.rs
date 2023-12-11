mod a_welcome;
mod b_to;
mod c_advent;
mod d_of;
mod e_rust;
mod f_;
mod g_I;
mod h_;
mod i_wish;
mod j_you;
mod k_a;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    crate::a_welcome::solution();
    let elapsed = now.elapsed();
    println!("Time elapsed: {:.2?}", elapsed);
    println!("----------");

    let now = Instant::now();
    crate::b_to::solution();
    let elapsed = now.elapsed();
    println!("Time elapsed: {:.2?}", elapsed);
    println!("----------");

    let now = Instant::now();
    crate::c_advent::solution();
    let elapsed = now.elapsed();
    println!("Time elapsed: {:.2?}", elapsed);
    println!("----------");

    let now = Instant::now();
    crate::d_of::solution();
    let elapsed = now.elapsed();
    println!("Time elapsed: {:.2?}", elapsed);
    println!("----------");

    let now = Instant::now();
    crate::e_rust::solution();
    let elapsed = now.elapsed();
    println!("Time elapsed: {:.2?}", elapsed);
    println!("----------");

    let now = Instant::now();
    crate::f_::solution();
    let elapsed = now.elapsed();
    println!("Time elapsed: {:.2?}", elapsed);
    println!("----------");

    let now = Instant::now();
    crate::g_I::solution();
    let elapsed = now.elapsed();
    println!("Time elapsed: {:.2?}", elapsed);
    println!("----------");

    let now = Instant::now();
    crate::h_::solution();
    let elapsed = now.elapsed();
    println!("Time elapsed: {:.2?}", elapsed);
    println!("----------");

    let now = Instant::now();
    crate::i_wish::solution();
    let elapsed = now.elapsed();
    println!("Time elapsed: {:.2?}", elapsed);
    println!("----------");

    let now = Instant::now();
    crate::j_you::solution();
    let elapsed = now.elapsed();
    println!("Time elapsed: {:.2?}", elapsed);
    println!("----------");

    let now = Instant::now();
    crate::k_a::solution();
    let elapsed = now.elapsed();
    println!("Time elapsed: {:.2?}", elapsed);
    println!("----------");
}
