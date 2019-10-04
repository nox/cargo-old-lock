use cargo_lock::Lockfile;
use serde::Serialize;
use toml::Serializer;

fn main() {
    let cargo_manifest_path = std::env::args_os().nth(1).expect("no manifest path");
    let lockfile = match Lockfile::load(&cargo_manifest_path) {
        Ok(lockfile) => lockfile,
        Err(e) => panic!("could not open manifest: {}", e),
    };
    let mut output = String::new();
    let mut serializer = Serializer::new(&mut output);
    serializer
        .pretty_array(true)
        .pretty_array_indent(1)
        .pretty_array_trailing_comma(true);
    if let Err(e) = lockfile.serialize(&mut serializer) {
        panic!("could not print manifest: {}", e)
    }
    let pretty_output = output
        .replace("dependencies = []\n", "")
        .replace("[\"", "[\n \"")
        .replace("\"]", "\",\n]");
    print!("{}\n{}", GENERATED.trim(), pretty_output);
}

const GENERATED: &'static str = r#"
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
"#;