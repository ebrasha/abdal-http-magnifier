mod abdal_http_magnifier;

use colored::Colorize;
use rustyline::{Editor};
use rustyline::history::DefaultHistory; // مسیر صحیح برای DefaultHistory

#[tokio::main]
async fn main() {
    // Enable ANSI coloring on Windows terminals
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).expect("ANSI support failed");

    abdal_http_magnifier::print_banner();

    let mut rl = Editor::<(), DefaultHistory>::new().expect("Failed to create line editor");

    loop {
        let input = rl.readline("\nEnter target URL (http or https): ");

        let target = match input {
            Ok(line) => {
                let trimmed = line.trim().to_string();
                if trimmed.is_empty() {
                    println!("{}", "[-] No input provided.".truecolor(255, 80, 80));
                    continue;
                }
                trimmed
            }
            Err(_) => {
                println!("{}", "[-] Failed to read input.".truecolor(255, 80, 80));
                continue;
            }
        };

        match abdal_http_magnifier::scan_http_methods(&target).await {
            Ok(_) => (),
            Err(e) => eprintln!("{}", format!("Error: {}", e).truecolor(255, 80, 80)),
        }

        let confirm = rl.readline("\nDo you want to scan another target? (y/n): ");
        match confirm {
            Ok(choice) => {
                if choice.trim().to_lowercase() != "y" {
                    println!("\n{}", "[+] Exiting Abdal HTTP Magnifier.".truecolor(159, 239, 0));
                    break;
                }
            }
            Err(_) => {
                println!("\n{}", "[-] Failed to read input. Exiting...".truecolor(255, 80, 80));
                break;
            }
        }
    }
}
