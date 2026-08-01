.global safety_guard_assembly
.type safety_guard_assembly, %function

// Função de guarda em Assembly ARM64
safety_guard_assembly:
    // Carrega o valor 1 (sucesso/autorizado) no registrador de retorno x0
    mov x0, #1
    
    // Retorna para a função chamadora em Rust
    ret
