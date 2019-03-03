#[macro_use]
extern crate afl;
extern crate goblin;

fn main() {
    fuzz!(|data: &[u8]| {
        goblin::elf::Elf::parse(&data);
    });
}
