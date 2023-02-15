use protobuf_codegen::Customize;

fn main() {
    cargo_emit::rerun_if_changed!("./build.rs");
    cargo_emit::rerun_if_changed!("./src/pbf/fileformat.proto");
    cargo_emit::rerun_if_changed!("./src/pbf/fileformat.rs");
    cargo_emit::rerun_if_changed!("./src/pbf/osmformat.proto");
    cargo_emit::rerun_if_changed!("./src/pbf/osmformat.rs");

    protobuf_codegen::Codegen::new()
        // Use `protoc` parser, optional.
        .pure()
        // Use `protoc-bin-vendored` bundled protoc command, optional.
        // .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(["./src/pbf"])
        // Inputs must reside in some of include paths.
        .inputs(["./src/pbf/fileformat.proto", "./src/pbf/osmformat.proto"])
        .out_dir("./src/pbf")
        .customize(Customize::default().gen_mod_rs(false))
        .run_from_script();
}
