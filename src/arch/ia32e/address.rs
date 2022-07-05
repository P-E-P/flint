use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VirtualAddress(u64);

impl VirtualAddress {
    /// Try to convert u64 into an ia32e virtual address.
    ///
    /// # Safety
    ///
    /// This function ensures the virtual address is correct.
    pub fn try_new(addr: u64) -> Result<Self, &'static str>
    {
        if addr < (1 << 48) {
            Ok(VirtualAddress(addr))
        } else {
            Err("Virtual address cannot be longer than 48 bits.")
        }
    }

    /// Convert u64 into an ia32e virtual address.
    ///
    /// # Safety
    ///
    /// This function panics if the virtual address is incorrect.
    pub fn new(addr: u64) -> Self
    {
        Self::try_new(addr).expect("Virtual address cannot be longer than 48 bits.")
    }
}

impl fmt::Debug for VirtualAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#X}", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PhysicalAddress(u64);

impl PhysicalAddress {
    /// Try to convert u64 into an ia32e physical address.
    ///
    /// # Safety
    ///
    /// This function ensures the physical address is correct.
    pub fn try_new(addr: u64) -> Result<Self, &'static str>
    {
        if addr < (1 << 52) {
            Ok(PhysicalAddress(addr))
        } else {
            Err("Physical address cannot be longer than 52 bits.")
        }
    }

    /// Convert u64 into an ia32e physical address.
    ///
    /// # Safety
    ///
    /// This function panics if the physical address is incorrect.
    pub fn new(addr: u64) -> Self
    {
        Self::try_new(addr).expect("Physical address cannot be longer than 52 bits.")
    }
}

impl fmt::Debug for PhysicalAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#X}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn virt_more_than_48() {
        let virt_addr = VirtualAddress::try_new(0x1_0000_0000_0000);
        assert!(virt_addr.is_err());
    }

    #[test_case]
    fn virt_less_than_48() {
        let virt_addr = VirtualAddress::try_new(0xFFFF_FFFF_FFFF);
        assert!(virt_addr.is_ok());
        assert_eq!(virt_addr.unwrap(), VirtualAddress::new(0xFFFF_FFFF_FFFF));
    }

    #[test_case]
    fn phys_more_than_52() {
        let phys_addr = PhysicalAddress::try_new(0x10_0000_0000_0000);
        assert!(phys_addr.is_err());
    }

    #[test_case]
    fn phys_less_than_52() {
        let phys_addr = PhysicalAddress::try_new(0xF_FFFF_FFFF_FFFF);
        assert!(phys_addr.is_ok());
        assert_eq!(phys_addr.unwrap(), PhysicalAddress::new(0xF_FFFF_FFFF_FFFF));
    }
}
