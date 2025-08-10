use risc0_zkvm::{ExecutorEnv, Executor, Prover};

fn main() {
    let env = ExecutorEnv::builder().build().unwrap();
    let code = include_bytes!("../../guest/target/guest/guest.risc0.zbin");
    let executor = Executor::from_elf(env, code).unwrap();

    let session = executor.run().unwrap();
    let receipt = session.prove().unwrap();

    assert!(receipt.verify().unwrap());
    println!("Proof verified!");
}
