use ia32::mm::gdt;

pub fn setup() {
    gdt::setup_gdt();
}
