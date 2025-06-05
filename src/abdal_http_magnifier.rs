// -------------------------------------------------------------------
// Programmer       : Ebrahim Shafiei (EbraSha)
// Email            : Prof.Shafiei@Gmail.com
// Telegram         : @ProfShafiei
// Github           : https://github.com/ebrasha
// -------------------------------------------------------------------
// This software is part of the Abdal arsenal, which belongs to the
// Abdal Security Group, led by Ebrahim Shafiei (EbraSha).
// -------------------------------------------------------------------

use reqwest::Client;
use std::collections::HashSet;
use std::time::Duration;
use colored::Colorize;

/// Display the banner with colors and author info
pub fn print_banner() {
    let banner = r#"

░█████╗░██████╗░██████╗░░█████╗░██╗░░░░░  ██╗░░██╗████████╗████████╗██████╗░
██╔══██╗██╔══██╗██╔══██╗██╔══██╗██║░░░░░  ██║░░██║╚══██╔══╝╚══██╔══╝██╔══██╗
███████║██████╦╝██║░░██║███████║██║░░░░░  ███████║░░░██║░░░░░░██║░░░██████╔╝
██╔══██║██╔══██╗██║░░██║██╔══██║██║░░░░░  ██╔══██║░░░██║░░░░░░██║░░░██╔═══╝░
██║░░██║██████╦╝██████╔╝██║░░██║███████╗  ██║░░██║░░░██║░░░░░░██║░░░██║░░░░░
╚═╝░░╚═╝╚═════╝░╚═════╝░╚═╝░░╚═╝╚══════╝  ╚═╝░░╚═╝░░░╚═╝░░░░░░╚═╝░░░╚═╝░░░░░

███╗░░░███╗░█████╗░░██████╗░███╗░░██╗██╗███████╗██╗███████╗██████╗░
████╗░████║██╔══██╗██╔════╝░████╗░██║██║██╔════╝██║██╔════╝██╔══██╗
██╔████╔██║███████║██║░░██╗░██╔██╗██║██║█████╗░░██║█████╗░░██████╔╝
██║╚██╔╝██║██╔══██║██║░░╚██╗██║╚████║██║██╔══╝░░██║██╔══╝░░██╔══██╗
██║░╚═╝░██║██║░░██║╚██████╔╝██║░╚███║██║██║░░░░░██║███████╗██║░░██║
╚═╝░░░░░╚═╝╚═╝░░╚═╝░╚═════╝░╚═╝░░╚══╝╚═╝╚═╝░░░░░╚═╝╚══════╝╚═╝░░╚═╝
 ver 1.2.1
    "#;

    println!("{}", banner.truecolor(187, 255, 52));
    println!("{}", "Handcrafted with Passion by Ebrahim Shafiei (EbraSha)".truecolor(159, 239, 0));
    println!("{}", "E-Mail   : Prof.Shafiei@Gmail.com".truecolor(159, 239, 0));
    println!("{}", "Telegram : @ProfShafiei".truecolor(159, 239, 0));
    println!("{}", "Github   : https://github.com/ebrasha".truecolor(159, 239, 0));
    println!("{}", "This software is part of the Abdal arsenal, which belongs to the Abdal Security Group, led by Ebrahim Shafiei (EbraSha).".truecolor(159, 239, 0));
}

/// Scan the supported HTTP methods of a given target
pub async fn scan_http_methods(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .danger_accept_invalid_certs(true)
        .build()?;

    let methods = vec![
        "GET", "POST", "PUT", "DELETE", "OPTIONS", "HEAD", "TRACE", "CONNECT", "PATCH",
    ];

    let mut open_methods = HashSet::new();

    for method in &methods {
        let request_builder = client
            .request(method.parse().unwrap(), target)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:124.0) Gecko/20100101 Firefox/124.0")
            .header("X-Forwarded-For", "127.0.0.1")
            .header("Sec-Fetch-Dest", "document")
            .header("Sec-Fetch-Mode", "navigate")
            .header("Accept-Language", "en-US,en;q=0.5")
            .header("Sec-Fetch-Site", "none")
            .header("Via", "1.1 evaded-proxy");

        let request = match *method {
            "POST" | "PUT" | "PATCH" => {
                // Add a basic body and Content-Type header for body-requiring methods
                request_builder
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body("key=test")
                    .build()?
            }
            _ => request_builder.build()?,
        };

        let response = client.execute(request).await;

        if let Ok(res) = response {
            let status = res.status().as_u16();
            if status < 400 {
                open_methods.insert(*method);
            }
        }
    }

    println!(
        "\n{} {}",
        "[+] Target scanned:".truecolor(255, 165, 0),
        target.truecolor(255, 165, 0)
    );

    if !open_methods.is_empty() {
        println!("{}", "\n[+] Open HTTP Methods Discovered:".truecolor(255, 165, 0));
        for method in &open_methods {
            println!("  {}", method.truecolor(1, 218, 205));
        }
    } else {
        println!(
            "{}",
            "[-] No open HTTP methods found or WAF/CDN is blocking.".truecolor(255, 80, 80)
        );
    }

    Ok(())
}
