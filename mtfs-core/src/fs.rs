use log::{info, warn, error, debug};

// pub mod metadata; why is it erroring out?

#[path = "metadata.rs"] 
pub mod metadata; 
// pub use metadataf as metadata;


pub fn format(data: &mut Vec<u8>, name: &str) {
    data.fill(0);
    // println!("{}", name);

    // Superblock Definition
    // 

    // Create a buffer to store the buffer in, 32 bytes max.. might be generous
    let mut name_buffer = [0u8; 32];
    let encoded_name = name.as_bytes();
    let encoded_name_length = encoded_name.len().min(32); // Count the length of the encoded name, max 32

    // From the name length of 0 to the length of the name ( max 32 ), copy the character byte into the name buffer
    name_buffer[0..encoded_name_length].copy_from_slice(&encoded_name[0..encoded_name_length]);

    let disk_size: u64 = (data.len() - 56) as u64;

    // Writing MTFS filesystem header
    data[0] = 0x6D; // m
    data[1] = 0x74; // t
    data[2] = 0x66; // f
    data[3] = 0x73; // s
    // I could use copy from slice to encode 'mtfs' but i just like how this looks

    // Copy name buffer into header
    data[4..36].copy_from_slice(&name_buffer);

    data[36..44].copy_from_slice(&disk_size.to_le_bytes());  // Convert the u64 into bytes w/ little endian. ( I just know LE is more common than BE )

    data[44] = 0x61;
    data[45] = 0x7A;
    data[46] = 0x75;
    data[47] = 0x6E;
    data[48] = 0x79;
    data[49] = 0x61;
    data[50] = 0x6E;
    data[51] = 0x6E;

    data[52] = 0x6D; // m
    data[53] = 0x74; // t
    data[54] = 0x66; // f
    data[55] = 0x73; // s

    // Dedicated 8MB region to metadata region marking
    // This basically in theory decreases write times as the library does not need to iterate through-
    // the _whole_ disk just to find a metadata block with free slots. The library will know ahead of time what blocks are occupied-
    // so it can skip them. Is it efficient... you're looking at someone's peculiar filesystem design so you tell me
    data[56..8388608].fill(0); 

    // Metadata block gen. logic
    // MTFS works by chunking both file data and general metadata eveningly throughout the disk.
    // Metadata chunks are 1048576 bytes in size ( 1mb ) and data regions are allocated 67108864 bytes ( 64mb )

    const METADATA_SIZE: u64 = (1024 * 1024); // 1mb

    let mut _metadata_region_count = 0;

    for offset in (8388608u64..disk_size).step_by(65*1024*1024 as usize) {
        let metadata_start = offset as usize;
        let metadata_end = (offset + METADATA_SIZE).min(disk_size) as usize;

        data[metadata_start..metadata_end].fill(0);
        _metadata_region_count += 1;
        println!("Written metadata chunk @ {}", metadata_start.to_string());
    }

    println!("Allocated metadata regions: {}", _metadata_region_count.to_string());

    println!("{}", disk_size.to_string());
    println!("{:?}", name_buffer);
}