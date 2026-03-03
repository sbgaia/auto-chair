use std::path::PathBuf;
use tokio::fs;

mod generator;

#[tokio::main]
async fn main() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("codegen crate must be in repo root")
        .to_path_buf();

    let schema_path = repo_root.join("schema/apc.json");
    let entity_output_path = repo_root.join("src/entities/apc.rs");

    let content = fs::read_to_string(&schema_path)
        .await
        .expect("failed to read schema/apc.json");
    let schema: serde_json::Value =
        serde_json::from_str(&content).expect("failed to parse schema/apc.json");

    let generated = generator::generate_entity(&schema)
        .expect("failed to generate LoadedApc from schema/apc.json");

    fs::write(&entity_output_path, generated)
        .await
        .expect("failed to write src/entities/apc.rs");

    println!(
        "Generated LoadedApc entity from {} to {}",
        schema_path.display(),
        entity_output_path.display()
    );
}
