fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let rc_path = std::path::Path::new(&manifest_dir).join("app.rc");
    embed_resource::compile(&rc_path, embed_resource::NONE);
}
