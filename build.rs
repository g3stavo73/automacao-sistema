fn main() {
    cc::Build::new()
        .file("src/program.asm")
        .compile("assembly_guard");
}
