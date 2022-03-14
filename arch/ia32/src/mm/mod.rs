pub mod gdt;

pub fn setup() {
    gdt::setup_gdt();
}
