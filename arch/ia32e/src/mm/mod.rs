pub mod gdt;

pub fn setup() {
    #[cfg(target_arch = "x86_64")]
    gdt::setup_gdt();
}
