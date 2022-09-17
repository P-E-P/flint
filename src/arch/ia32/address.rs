use core::fmt;
use core::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VirtualAddress(u64);

impl VirtualAddress {
    fn is_canonical(addr: u64) -> bool {
        addr < (1 << 47) || (addr >> 47) == 0x1FFFF
    }

    /// Try to convert u64 into an ia32e virtual address.
    ///
    /// # Safety
    ///
    /// This function ensures the virtual address is correct.
    pub fn try_new(addr: u64) -> Result<Self, &'static str> {
        if Self::is_canonical(addr) {
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
    pub fn new(addr: u64) -> Self {
        Self::try_new(addr).expect("Virtual address cannot be longer than 48 bits.")
    }

    /// Convert a Rust function into an ia32e virtual address.
    pub fn from_fn(src: extern "x86-interrupt" fn()) -> Self {
        Self(src as *const() as u64)
    }

    /// Convert u64 into an ia32e virtual address.
    ///
    /// # Safety
    ///
    /// This function avoids any check and is therefore unsafe.
    unsafe fn unchecked_new(addr: u64) -> Self {
        VirtualAddress(addr)
    }
}

impl Add for VirtualAddress {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl AddAssign for VirtualAddress {
    fn add_assign(&mut self, other: Self) {
        (*self).0 = (*self).0 + other.0;
    }
}

impl Sub for VirtualAddress {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl SubAssign for VirtualAddress {
    fn sub_assign(&mut self, other: Self) {
        (*self).0 = (*self).0 - other.0;
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
    pub fn try_new(addr: u64) -> Result<Self, &'static str> {
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
    pub fn new(addr: u64) -> Self {
        Self::try_new(addr).expect("Physical address cannot be longer than 52 bits.")
    }

    /// Convert u64 into an ia32e physical address.
    ///
    /// # Safety
    ///
    /// This function avoids any check and is therefore unsafe.
    unsafe fn unchecked_new(addr: u64) -> Self {
        PhysicalAddress(addr)
    }
}

impl Add for PhysicalAddress {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl AddAssign for PhysicalAddress {
    fn add_assign(&mut self, other: Self) {
        (*self).0 = (*self).0 + other.0;
    }
}

impl Sub for PhysicalAddress {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl SubAssign for PhysicalAddress {
    fn sub_assign(&mut self, other: Self) {
        (*self).0 = (*self).0 - other.0;
    }
}

impl Into<u64> for VirtualAddress {
    fn into(self) -> u64 {
        self.0
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
    fn virt_upper_canonical() {
        let virt_addr = VirtualAddress::try_new(0xFFFF_8000_0000_0000);
        assert!(virt_addr.is_ok());
    }

    #[test_case]
    fn virt_lower_canonical() {
        let virt_addr = VirtualAddress::try_new(0x7FFF_FFFF_FFFF);
        assert!(virt_addr.is_ok());
    }

    #[test_case]
    fn virt_not_canonical() {
        let virt_addr = VirtualAddress::try_new(0x1_0000_0000_0000);
        assert!(virt_addr.is_err());
    }

    #[test_case]
    fn virt_not_canonical_2() {
        let virt_addr = VirtualAddress::try_new(0xFFFF_0000_0000_0000);
        assert!(virt_addr.is_err());
    }

    #[test_case]
    fn virt_unsafe_wrong() {
        unsafe {
            let virt_addr = VirtualAddress::unchecked_new(0x1_0000_0000_0000);
            assert_eq!(virt_addr.0, 0x1_0000_0000_0000);
        }
    }

    #[test_case]
    fn virt_add() {
        let virt1 = VirtualAddress::new(1);
        let virt2 = VirtualAddress::new(3);
        assert_eq!(VirtualAddress::new(4), virt1 + virt2);
    }

    #[test_case]
    fn virt_add_assign() {
        let mut virt1 = VirtualAddress::new(1);
        virt1 += VirtualAddress::new(3);
        assert_eq!(VirtualAddress::new(4), virt1);
    }

    #[test_case]
    fn virt_sub() {
        let virt1 = VirtualAddress::new(1);
        let virt2 = VirtualAddress::new(3);
        assert_eq!(VirtualAddress::new(2), virt2 - virt1);
    }

    #[test_case]
    fn virt_sub_assign() {
        let mut virt1 = VirtualAddress::new(4);
        virt1 -= VirtualAddress::new(3);
        assert_eq!(VirtualAddress::new(1), virt1);
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

    #[test_case]
    fn phys_unsafe_wrong() {
        unsafe {
            let phys_addr = PhysicalAddress::unchecked_new(0x10_0000_0000_0000);
            assert_eq!(phys_addr.0, 0x10_0000_0000_0000);
        }
    }

    #[test_case]
    fn phys_add() {
        let phys1 = PhysicalAddress::new(1);
        let phys2 = PhysicalAddress::new(3);
        assert_eq!(PhysicalAddress::new(4), phys1 + phys2);
    }

    #[test_case]
    fn phys_add_assign() {
        let mut phys1 = PhysicalAddress::new(1);
        phys1 += PhysicalAddress::new(3);
        assert_eq!(PhysicalAddress::new(4), phys1);
    }

    #[test_case]
    fn phys_sub() {
        let phys1 = PhysicalAddress::new(1);
        let phys2 = PhysicalAddress::new(3);
        assert_eq!(PhysicalAddress::new(2), phys2 - phys1);
    }

    #[test_case]
    fn phys_sub_assign() {
        let mut phys1 = PhysicalAddress::new(4);
        phys1 -= PhysicalAddress::new(3);
        assert_eq!(PhysicalAddress::new(1), phys1);
    }
}
