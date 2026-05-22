use bootloader::DiskImageBuilder;
use std::path::PathBuf;

fn main() {
    let kernel = PathBuf::from("../../target/x86_64-unikernel/debug/ember");

    let out_path = PathBuf::from("../../boot-bios.img");

    DiskImageBuilder::new(kernel)
        .create_bios_image(&out_path)
        .expect("failed to create BIOS image");

    println!("Boot image created: boot-bios.img");
}