use std::path::PathBuf;
use tokio::fs;

mod generator;

#[tokio::main]
async fn main() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("codegen crate must be in repo root")
        .to_path_buf();

    let schema_dir = repo_root.join("schema");
    let entity_output_dir = repo_root.join("src/entities");

    let mut entries = fs::read_dir(&schema_dir)
        .await
        .expect("failed to read schema directory");

    loop {
        let entry = entries.next_entry()
            .await
            .expect("failed to read schema directory entry");
        
        if let None = entry {
            break;
        }

        let entry = entry.unwrap();
        if entry.path().extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        let file_stem = entry.path().file_stem().expect("schema file must have a name").to_os_string();
        let file_name = file_stem.to_str().expect("schema file name must be valid UTF-8");

        println!("Processing schema file: {}", file_name);
        let content = fs::read_to_string(entry.path())
            .await
            .expect("failed to read schema file");

        let schema: serde_json::Value = serde_json::from_str(&content)
            .expect("failed to parse schema JSON");
        let entity_code = generator::generate_entity(&schema).expect("failed to generate entity code");

        let output_path = entity_output_dir.join(format!("{file_name}.rs"));
        fs::write(&output_path, entity_code)
            .await
            .expect("failed to write entity file");
    }
}