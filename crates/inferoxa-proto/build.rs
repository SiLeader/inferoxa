fn main() {
    #[cfg(feature = "__proto")]
    build_proto(
        "inferoxa.v1.bin",
        &[
            #[cfg(feature = "embeddings")]
            "../../proto/inferoxa/v1/embeddings.proto",
            #[cfg(feature = "models")]
            "../../proto/inferoxa/v1/models.proto",
        ],
    );
}

#[cfg(feature = "__proto")]
fn build_proto(fd: &str, protos: &[&str]) {
    let builder = tonic_prost_build::configure()
        .build_server(cfg!(feature = "server"))
        .build_client(cfg!(feature = "client"));
    let builder = if cfg!(feature = "server") {
        let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
        builder.file_descriptor_set_path(out_dir.join(fd))
    } else {
        builder
    };

    builder.compile_protos(protos, &["../../proto"]).unwrap();
}
