fn main() {
    tonic_build::compile_protos("proto/board.proto").unwrap();
}
