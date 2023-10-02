use cuda_builder::CudaBuilder;

fn main() {
    CudaBuilder::new("../gpu").build().unwrap();
    // CudaBuilder::new("../gpu").copy_to("some/path.ptx").build().unwrap();
}