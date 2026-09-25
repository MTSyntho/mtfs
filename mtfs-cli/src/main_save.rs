use mtfs::*;

fn main() {
    let mut storage = vec![0u8; 8388608]; // 8 MiB test filesystem
    mtfs::fs::format(&mut storage, "AXIUMCRISIS");

    println!("Disk name: {}", mtfs::fs::metadata::get_name(&storage));
    println!("Disk size: {}MB", mtfs::fs::metadata::get_size_mb(&storage));

    std::fs::write("mtfs.img", &storage).expect("Failed to write MTFS image");
}
