use log::{info, warn, error, debug};

pub fn get_name(data: &Vec<u8>) -> String {
    let name_buffer = &data[4..36];
    std::str::from_utf8(&name_buffer).expect("Invalid UTF-8").to_string()

}

pub fn get_size(data: &Vec<u8>) -> u64 {
    let size_buffer: [u8; 8] = data[36..44].try_into().unwrap();
    let disk_size: u64 = u64::from_le_bytes(size_buffer);
    disk_size
}

pub fn get_size_mb(data: &Vec<u8>) -> f32 {
    let size = get_size(&data) as f32 / 1048576.0;
    (size * 100.0).trunc() / 100.0 // Truncate down to 2 decimal places... with strange math operations
}