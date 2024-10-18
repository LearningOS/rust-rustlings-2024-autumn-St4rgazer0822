/// # Safety
///
/// The `address` must contain a valid and mutable reference to a `u32`.
/// The caller must ensure that the address is correctly derived from a valid
/// `u32` reference and that the address is not used elsewhere concurrently.
unsafe fn modify_by_address(address: usize) {
    // SAFETY: We assume that the `address` is valid and points to a `u32` value
    // as per the contract of this function.
    unsafe {
        let ptr = address as *mut u32;
        *ptr = 0xAABBCCDD; // Modify the value at the address
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
