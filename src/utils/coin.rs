/**
 * Formats a coin address by padding the hexadecimal part to 64 characters.
 * 
 * If the input string contains "::", it splits the string at that position,
 * takes the part before "::" as a hexadecimal address, pads it to 64 characters,
 * and then rejoins it with the part after "::".
 * 
 * If the input doesn't contain "::", it returns the original string unchanged.
 * 
 * @params - The coin address string to format
 * @return The formatted coin address
 */
pub fn format_coin_address(s: &str) -> String {
    if !s.contains("::") {
        return s.to_string();
    }

    if let Some(pos) = s.find("::") {
        let (hex_part, rest) = s.split_at(pos);
        
        let hex_str = hex_part.strip_prefix("0x").unwrap_or(hex_part);
        let padded_hex_str = format!("{:0>64}", hex_str);
        
        format!("0x{}{}", padded_hex_str, rest)
    } else {
        s.to_string()
    }
}
