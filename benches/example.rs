use std::hint::black_box;

use rust_gha_workflows_demo::add;

fn main() {
    divan::main();
}

#[divan::bench]
fn example() -> u64 {
    add(black_box(2), black_box(2))
}
