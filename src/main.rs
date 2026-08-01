use std::process::Command;
use std::io::{self, Write};

fn main() {
    println!("=== Sistema de Automacao de Baixo Nivel (Rust) ===");
    println!("1. Executar verificacao de sistema");
    println!("2. Sair");
    print!("Escolha uma opcao: ");
    
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim();

    match choice {
        "1" => {
            println!("\nExecutando comandos do sistema...");
            
            // Exemplo de chamada nativa (equivalente a interagir com o ambiente do terminal)
            let output = Command::new("uname")
                .arg("-a")
                .output();

            match output {
                Ok(res) => {
                    let system_info = String::from_utf8_lossy(&res.stdout);
                    println!("Informacoes do Kernel/Sistema: {}", system_info);
                }
                Err(e) => eprintln!("Erro ao executar comando: {}", e),
            }
        }
        "2" => {
            println!("Saindo...");
        }
        _ => {
            println!("Opcao invalida.");
        }
    }
}
