use ark_bn254::Bn254;
use crate::circuit::MyCircuit;
use crate::proof::generate_proof;
use crate::vk::load_verification_key;

fn main() {
    let circuit = MyCircuit { a: Some(5), b: Some(3) };

    // Run Groth16 proof generation and verification for this circuit
    let proof = generate_proof::<Bn254>(circuit);
    let vk = load_verification_key();
    
    assert!(verify_proof(proof, vk));
}
