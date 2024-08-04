use nexus_sdk::{
    compile::CompileOpts,
    nova::seq::{Generate, Nova, PP},
    Local, Prover, Verifiable,
};

const PACKAGE: &str = "guest";

fn main() {
    let mut opts = CompileOpts::new(PACKAGE);
    opts.set_memlimit(8); // use an 8mb memory

    println!("Compiling guest program...");
    let prover: Nova<Local> = Nova::compile(&opts).expect("failed to compile guest program");

    println!("Setting up Nova public parameters...");
    let pp: PP = PP::generate().expect("failed to generate parameters");

    println!("Proving execution of vm...");
    let input: u32 = 0xdeadbeef;
    let proof = prover
        .prove_with_input(&pp, &input)
        .expect("failed to prove program");

    println!(">>>>> Logging\n{}<<<<<", proof.logs().join(""));

    print!("Verifying execution...");
    proof.verify(&pp).expect("failed to verify proof");

    println!("  Succeeded!");
}
