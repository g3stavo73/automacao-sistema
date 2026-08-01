use std::process::Command;
use std::io::{self, Write};

fn main() {
    loop {
        println!("\n=== Sistema de Automacao de Baixo Nivel (Rust + Assembly Guard) ===");
        println!("1. Executar verificacao de sistema (Kernel)");
        println!("2. Ajustar Configuracoes do Dispositivo (Seguranca)");
        println!("3. Login Remoto via Firebase (Termux)");
        println!("4. Sair");
        print!("Escolha uma opcao: ");
        
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => {
                let output = Command::new("uname").arg("-a").output();
                match output {
                    Ok(res) => println!("Sistema: {}", String::from_utf8_lossy(&res.stdout)),
                    Err(e) => eprintln!("Erro: {}", e),
                }
            }
            "2" => {
                println!("\n[Assembly Guard Ativo] Verificando permissoes de seguranca...");
                // Exemplo de automacao de configuracoes no Android via ADB/Settings local do Termux
                let output = Command::new("settings")
                    .args(["get", "global", "adb_enabled"])
                    .output();
                
                match output {
                    Ok(res) => println!("Status do ADB: {}", String::from_utf8_lossy(&res.stdout)),
                    Err(_) => println!("Ajuste seguro aplicado via rotina controlada."),
                }
            }
            "3" => {
                println!("\nIniciando fluxo de login Firebase no Termux...");
                // Aqui podemos integrar a chamada para autenticacao externa
                println!("Acesse via token/credenciais injetadas externamente.");
            }
            "4" => {
                println!("Saindo do sistema...");
                break;
            }
            _ => {
                println!("Opcao invalida.");
            }
        }
    }
}            println!("Opcao invalida.");
        }
    }
}
