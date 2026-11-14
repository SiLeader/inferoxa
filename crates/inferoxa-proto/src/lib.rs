#[cfg(feature = "__proto")]
pub use prost;
#[cfg(feature = "__proto")]
pub use prost_types;
#[cfg(feature = "__proto")]
pub use tonic;

#[cfg(feature = "__proto")]
pub mod inferoxa {
    pub mod v1 {
        tonic::include_proto!("inferoxa.v1");

        #[cfg(feature = "server")]
        const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("inferoxa.v1.bin");
    }
}
