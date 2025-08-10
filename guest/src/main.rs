use risc0_zkvm::guest::env;

fn main() {
    let a = 5;
    let b = 7;
    let sum = a + b;
    env::commit_u32(sum);
}
