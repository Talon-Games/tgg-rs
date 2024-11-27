pub fn calculate_checksum(bytes: Vec<u8>) -> [u8; 2] {
    let checksum = bytes
        .iter()
        .map(|&byte| byte as u32)
        .sum::<u32>()
        .to_le_bytes()[0..2]
        .to_vec();

    return [checksum[0], checksum[1]];
}

pub fn extract_cstring(bytes: &[u8]) -> String {
    bytes
        .iter()
        .take_while(|&&byte| byte != 0x00)
        .map(|&byte| byte as char)
        .collect()
}

pub fn extract_cstring_with_offset(bytes: &[u8], start: usize) -> (String, usize) {
    let mut end = start;
    while end < bytes.len() && bytes[end] != 0x00 {
        end += 1;
    }
    let result = bytes[start..end]
        .iter()
        .map(|&byte| byte as char)
        .collect::<String>();
    (result, end + 1) // +1 to skip the null terminator
}
