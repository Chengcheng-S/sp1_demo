use sp1_helper::build_program;

fn main() {
    build_program(&format!("{}/../program", env!("CARGO_MANIFEST_DIR")));
}
