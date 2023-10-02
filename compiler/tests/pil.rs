use backend::BackendType;
use number::{Bn254Field, GoldilocksField};
use std::path::Path;
use test_log::test;

pub fn verify_pil(file_name: &str, query_callback: Option<fn(&str) -> Option<GoldilocksField>>) {
    let input_file = Path::new(&format!(
        "{}/../test_data/pil/{file_name}",
        env!("CARGO_MANIFEST_DIR")
    ))
    .canonicalize()
    .unwrap();

    let temp_dir = mktemp::Temp::new_dir().unwrap();
    assert!(compiler::compile_pil(
        &input_file,
        &temp_dir,
        query_callback,
        Some(BackendType::PilStarkCli)
    )
    .witness
    .is_some());
    compiler::verify(&temp_dir);
}

fn gen_estark_proof(file_name: &str, inputs: Vec<GoldilocksField>) {
    compiler::compile_pil_or_asm(
        format!(
            "{}/../test_data/pil/{file_name}",
            env!("CARGO_MANIFEST_DIR")
        )
        .as_str(),
        inputs,
        &mktemp::Temp::new_dir().unwrap(),
        true,
        Some(BackendType::EStark),
    )
    .unwrap();
}

#[cfg(feature = "halo2")]
fn gen_halo2_proof(file_name: &str, inputs: Vec<Bn254Field>) {
    compiler::compile_pil_or_asm(
        format!(
            "{}/../test_data/pil/{file_name}",
            env!("CARGO_MANIFEST_DIR")
        )
        .as_str(),
        inputs,
        &mktemp::Temp::new_dir().unwrap(),
        true,
        Some(BackendType::Halo2),
    )
    .unwrap();
}

#[cfg(not(feature = "halo2"))]
fn gen_halo2_proof(_file_name: &str, _inputs: Vec<Bn254Field>) {}

#[test]
fn test_fibonacci() {
    let f = "fibonacci.pil";
    verify_pil(f, None);
    gen_halo2_proof(f, Default::default());
    gen_estark_proof(f, Default::default());
}

#[test]
fn test_constant_in_identity() {
    let f = "constant_in_identity.pil";
    verify_pil(f, None);
    gen_halo2_proof(f, Default::default());
    gen_estark_proof(f, Default::default());
}

#[test]
fn test_fibonacci_macro() {
    let f = "fib_macro.pil";
    verify_pil(f, None);
    gen_halo2_proof(f, Default::default());
    gen_estark_proof(f, Default::default());
}

#[test]
fn test_global() {
    verify_pil("global.pil", None);
    // Halo2 would take too long for this.
    // Starky requires at least one witness column, this test has none.
}

#[test]
fn test_sum_via_witness_query() {
    verify_pil(
        "sum_via_witness_query.pil",
        Some(|q| {
            match q {
                "\"in\", 0" => Some(7.into()),
                "\"in\", 1" => Some(8.into()),
                "\"in\", 2" => Some(2.into()),
                "\"in\", 3" => None, // This line checks that if we return "None", the system still tries to figure it out on its own.
                _ => None,
            }
        }),
    );
    // prover query string uses a different convention,
    // so we cannot directly use the halo2_proof and estark functions here.
}

#[test]
fn test_witness_lookup() {
    let f = "witness_lookup.pil";
    verify_pil(
        f,
        Some(|q| match q {
            "\"input\", 0" => Some(3.into()),
            "\"input\", 1" => Some(5.into()),
            "\"input\", 2" => Some(2.into()),
            _ => Some(7.into()),
        }),
    );
    // halo2 fails with "gates must contain at least one constraint"
    let inputs = vec![3, 5, 2, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7];
    gen_estark_proof(f, inputs.into_iter().map(GoldilocksField::from).collect());
}

#[test]
#[should_panic(expected = "Witness generation failed.")]
fn test_underdetermined_zero_no_solution() {
    verify_pil("underdetermined_zero_no_solution.pil", None);
}

#[test]
fn test_pair_lookup() {
    let f = "pair_lookup.pil";
    verify_pil(f, None);
    // halo2 would take too long for this
    // starky would take too long for this in debug mode
}

#[test]
fn test_block_lookup_or() {
    let f = "block_lookup_or.pil";
    verify_pil(f, None);
    // halo2 would take too long for this
    // starky would take too long for this in debug mode
}

#[test]
fn test_halo_without_lookup() {
    let f = "halo_without_lookup.pil";
    verify_pil(f, None);
    gen_halo2_proof(f, Default::default());
    gen_estark_proof(f, Default::default());
}

#[test]
fn test_simple_div() {
    let f = "simple_div.pil";
    verify_pil(f, None);
    // starky would take too long for this in debug mode
}

#[test]
fn test_single_line_blocks() {
    let f = "single_line_blocks.pil";
    verify_pil(f, None);
    gen_estark_proof(f, Default::default());
}

#[test]
fn test_two_block_machine_functions() {
    let f = "two_block_machine_functions.pil";
    verify_pil(f, None);
    gen_estark_proof(f, Default::default());
}

#[test]
fn test_fixed_columns() {
    let f = "fixed_columns.pil";
    verify_pil(f, None);
    // Starky requires at least one witness column, this test has none.
}

#[test]
fn test_witness_via_let() {
    verify_pil("witness_via_let.pil", None);
}
