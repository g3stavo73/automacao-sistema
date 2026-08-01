fn main() {
    // Compila o arquivo Assembly para a arquitetura AArch64
    cc::Build::new()
        .file("src/program.asm")
        .compile("assembly_guard");
}
