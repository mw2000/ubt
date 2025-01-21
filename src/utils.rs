use alloy_primitives::Address;

/// Converts an `Address` (20 bytes) to a `[u8; 32]` by zero-padding on the left.
/// This is commonly used in the proposal to embed an account address into the 32-byte key space.
pub fn address_to_32(address: Address) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[12..].copy_from_slice(address.as_slice());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_conversion() {
        let addr = Address::from([0x11u8; 20]);
        let arr32 = address_to_32(addr);
        assert_eq!(&arr32[12..], &[0x11u8; 20]);
    }
}
