use std::{error::Error, time::Duration};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let mut port = serialport::new("/dev/ttyACM0", 115200)
        .timeout(Duration::from_millis(1000))
        .open()?;

    ui.on_command_sent({
        let ui_handle = ui.as_weak();

        move || {
            let ui = ui_handle.unwrap();
            let cmd = ui.get_command();

            if !cmd.is_empty() {
                ui.set_interface(ui.get_interface() + &format!("Sent: {}\n", cmd));
                ui.set_enable_command_sending(false);

                match port.write_all(format!("{}\n", cmd).as_bytes()) {
                    Ok(_) => {
                        let mut response = String::new();
                        let mut buffer = [0u8; 128];

                        loop {
                            match port.read(&mut buffer) {
                                Ok(n) if n > 0 => {
                                    let chunk = String::from_utf8_lossy(&buffer[..n]);
                                    response.push_str(&chunk);

                                    // stop when newline received
                                    if response.contains('\n') {
                                        break;
                                    }
                                }
                                Ok(_) => {
                                    // nothing read, keep waiting
                                }
                                Err(e) => {
                                    ui.set_interface(
                                        ui.get_interface() + &format!("Read Error: {}\n", e),
                                    );
                                    break;
                                }
                            }
                        }

                        ui.set_interface(ui.get_interface() + &format!("{}\n", response));
                    }

                    Err(e) => {
                        ui.set_interface(ui.get_interface() + &format!("Error Writing: {}\n", e));
                    }
                }

                ui.set_enable_command_sending(true);
            }
        }
    });

    ui.run()?;

    Ok(())
}
