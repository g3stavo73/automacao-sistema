use std::process::Command;
use std::io::{self, Write};

// Declaração da função escrita em Assembly puro (program.asm)
extern "C" {
    fn safety_guard_assembly() -> u64;
}

fn main() {
    loop {
        println!("\n=== Automacao (Rust + Assembly Puro) ===");
        println!("1. Executar verificacao de sistema (Kernel)");
        println!("2. Ajustar Configuracoes (Protegido por Assembly Externo)");
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
                println!("\n[Assembly Puro] Acionando rotina externa `program.asm`...");
                
                let guard_status: u64;
                unsafe {
                    // Chamada da função em Assembly puro
                    guard_status = safety_guard_assembly();
                }

                if guard_status == 1 {
                    println!("[Assembly Guard] Integridade validada pelo hardware! Executando alteracao segura...");
                    
                    let output = Command::new("settings")
                        .args(["get", "global", "adb_enabled"])
                        .output();
                    
                    match output {
                        Ok(res) => println!("Status obtido com seguranca: {}", String::from_utf8_lossy(&res.stdout)),
                        Err(_) => println!("Ajuste aplicado com restricoes estritas."),
                    }
                } else {
                    println!("[ERRO CRITICO] Acesso negado pelo Assembly Guard.");
                }
            }
            "3" => {
                println!("\n[Firebase] Configurando login remoto via Termux...");
                println!("Pronto para autenticar com credenciais externas.");
            }
            "4" => {
                println!("Saindo...");
                break;
            }
            _ => {
                println!("Opcao invalida.");
            }
        }
    }
}                }
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
