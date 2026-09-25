use mtfs::*;

fn main() {
    // let mut storage = vec![0u8; 8388608]; // 8mb test file in memory, cant be bothered to do std::io rn
    let mut storage = vec![0u8; 134217728]; // 128mb test file in memory, still cant be bothered to do std::io rn
    mtfs::fs::format(&mut storage, "AXIUMCRISIS");
    // mtfs::fs::format(&mut storage, "こんにちわ みんなさん");

    println!("Disk name: {}", mtfs::fs::metadata::get_name(&storage));
    println!("Disk size: {}MB", mtfs::fs::metadata::get_size_mb(&storage));
}
