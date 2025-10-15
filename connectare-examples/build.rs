use connectare_build::{connectare_codegen, AxumConnectGenSettings};

fn main() {
    let settings = AxumConnectGenSettings::from_directory_recursive("proto")
        .expect("failed to glob proto files");
    connectare_codegen(settings).unwrap();
}
