pub mod gdt;
pub mod descriptor;

pub fn setup() {
    #[cfg(target_arch = "x86_64")]
    gdt::setup_gdt();
}
