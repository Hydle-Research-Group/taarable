use clap::{Command, arg};
use serde::Deserialize;
use serialport::SerialPort;
use serialport::SerialPortType;
use std::fs;
use std::io::Write;
use std::{collections::HashMap, time::Duration};

#[derive(Deserialize)]
struct JsonResponse(HashMap<String, String>);

fn main() {
    let matches = Command::new("TAARABLE")
        .version("0.1.0")
        .about("A terminal interface for USB communication with the TAAR robotic arm")
        .arg(arg!(--port <USB_NAME>).help("Specify a USB port to use"))
        .arg(arg!(--list ...).help("List available ports"))
        .arg(arg!(--upload <FILE>).help("Upload a GCODE sequence"))
        .get_matches();

    if let Some(g) = matches.get_one::<u8>("list")
        && *g == 1u8
    {
        let ports = serialport::available_ports().unwrap();

        for p in ports {
            match p.port_type {
                SerialPortType::Unknown => {
                    println!("Port: `{}` Status: Unknown", p.port_name)
                }
                _ => {
                    println!("Port: `{}` Status: Available", p.port_name)
                }
            }
        }

        return;
    }

    let uart_port: Option<Box<dyn SerialPort>>;

    match matches.get_one::<String>("port") {
        Some(p) => {
            match serialport::new(p, 115200)
                .timeout(Duration::from_millis(1000))
                .open()
            {
                Ok(port) => {
                    println!("Successfully connected to port: {}", port.name().unwrap());

                    uart_port = Some(port);
                }
                Err(e) => {
                    println!("Failed to open port: {}", e);
                    return;
                }
            }
        }
        _ => {
            return;
        }
    }

    if let Some(mut uart_port) = uart_port {
        match matches.get_one::<String>("upload") {
            Some(p) => {
                let contents = fs::read_to_string(p).unwrap_or_else(|e| {
                    println!("Error uploading file: {e}");

                    String::new()
                });
                let data = contents.split("\n").collect::<Vec<&str>>();

                for cmd in data {
                    match send_command(&mut uart_port, cmd) {
                        Err(e) => {
                            if !e.is_empty() {
                                println!("{e}");
                            } else {
                                return;
                            }
                        }
                        _ => {}
                    }
                }

                println!("Sequence finished");

                return; // skip REPL, file uploaded
            }
            _ => {}
        }

        loop {
            let mut code = String::new();

            print!(">>> ");
            let _ = std::io::stdout().flush();

            std::io::stdin()
                .read_line(&mut code)
                .expect("Input text (stdin) was not a valid string");

            if let Err(e) = send_command(&mut uart_port, &code)
                && !e.is_empty()
            {
                println!("{e}");
            }
        }
    }
}

fn send_command(port: &mut Box<dyn SerialPort>, command: &str) -> Result<(), String> {
    match port.write_all(format!("{}\n", command).as_bytes()) {
        Ok(_) => {
            let mut response = String::new();
            let mut buffer = [0u8; 128];

            loop {
                match port.read(&mut buffer) {
                    Ok(n) if n > 0 => {
                        let chunk = String::from_utf8_lossy(&buffer[..n]);

                        if response.contains('\n') {
                            break;
                        }

                        response.push_str(&chunk);
                    }
                    Ok(_) => continue,
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                        break;
                    }
                    Err(e) => {
                        return Err(format!("Read Error: {}", e));
                    }
                }
            }

            if !response.is_empty() {
                let received: JsonResponse = match serde_json::from_str(&response) {
                    Ok(r) => r,
                    Err(e) => {
                        return Err(format!("Error parsing response: {}", e));
                    }
                };

                for (kind, message) in received.0 {
                    if kind == "info" {
                        println!("Info: {}", message);
                    } else if kind == "warning" {
                        println!("Warning: {}", message);
                    } else if kind == "error" {
                        return Err(format!("Error: {}", message));
                    } else if kind == "queue" {
                        if message == "quit" {
                            return Err("".to_string());
                        }
                    }
                }
            }
        }
        Err(e) => {
            return Err(format!("Error writing: {}", e));
        }
    }

    Ok(())
}
