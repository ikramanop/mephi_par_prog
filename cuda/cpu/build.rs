use cuda_builder::CudaBuilder;

fn main() {
    CudaBuilder::new("../gpu")
        .copy_to("../resources/calc.ptx")
        .build()
        .unwrap();
}
