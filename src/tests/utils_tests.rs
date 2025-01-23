use super::*;
use alloy_primitives::Address;

#[test]
fn test_address_to_32_zero_padding() {
    let addr = Address::from([0x42; 20]);
    let key = address_to_32(addr);

    // First 12 bytes should be zero
    assert_eq!(&key[..12], &[0; 12]);
    // Last 20 bytes should match address
    assert_eq!(&key[12..], &[0x42; 20]);
}

#[test]
fn test_address_to_32_roundtrip() {
    let original = Address::from([0x42; 20]);
    let key = address_to_32(original);
    let recovered = Address::from_slice(&key[12..]);

    assert_eq!(original, recovered);
}
