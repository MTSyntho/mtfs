use log::{info, warn, error, debug};

pub fn format(data: &mut Vec<u8>, name: &str) {
    data.fill(0);
    // println!("{}", name);

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

    println!("{}", disk_size.to_string());
    println!("{:?}", name_buffer);
}