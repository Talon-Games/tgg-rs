pub fn calculate_checksum(bytes: Vec<u8>) -> [u8; 2] {
    let checksum = bytes
        .iter()
        .map(|&byte| byte as u32)
        .sum::<u32>()
        .to_le_bytes()[0..2]
        .to_vec();

    return [checksum[0], checksum[1]];
}
