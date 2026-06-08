use serialport::{SerialPort, SerialPortType::UsbPort, available_ports};
use slint::{ModelRc, SharedString, VecModel};
use std::{cell::RefCell, error::Error, rc::Rc};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let serial_port: Rc<RefCell<Option<Box<dyn SerialPort>>>> = Rc::new(RefCell::new(None));
    let mut usb_ports = available_ports().expect("No available ports");
    usb_ports.sort_by_key(|p| match p.port_type {
        UsbPort(_) => 0,
        _ => 1,
    });
    let ports: ModelRc<SharedString> = ModelRc::new(Rc::new(VecModel::from(
        usb_ports
            .iter()
            .map(|p| p.port_name.clone().into())
            .collect::<Vec<SharedString>>(),
    )));

    // default ports
    ui.set_ports(ports);

    ui.on_command_sent({
        let ui_handle = ui.as_weak();
        let p = serial_port.clone();

        move || {
            let ui = ui_handle.unwrap();
            let cmd = ui.get_command();

            if !cmd.is_empty() {
                ui.set_interface(ui.get_interface() + &format!("Sent: {}\n", cmd));
                ui.set_enable_command_sending(false);

                let mut port_ref = p.borrow_mut();

                if let Some(port) = port_ref.as_mut() {
                    match port.write_all(format!("{}\n", cmd).as_bytes()) {
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
                                        ui.set_interface(
                                            ui.get_interface() + &format!("Read Error: {}\n", e),
                                        );
                                        break;
                                    }
                                }
                            }

                            ui.set_interface(ui.get_interface() + &format!("{}", response));
                        }

                        Err(e) => {
                            ui.set_interface(
                                ui.get_interface() + &format!("Error Writing: {}\n", e),
                            );
                        }
                    }
                } else {
                    ui.set_interface(ui.get_interface() + "No serial port selected/open\n");
                }

                ui.set_enable_command_sending(true);
            }
        }
    });

    ui.on_port_changed({
        let ui_handle = ui.as_weak();
        let p = serial_port.clone();

        move || {
            let ui = ui_handle.unwrap();

            match serialport::new(&*ui.get_active_port(), 115200).open() {
                Ok(port) => {
                    *p.borrow_mut() = Some(port);

                    ui.set_interface(
                        ui.get_interface() + &format!("Connected to {}\n", ui.get_active_port()),
                    );
                }

                Err(e) => {
                    ui.set_interface(ui.get_interface() + &format!("Failed to open port: {}\n", e));
                }
            }
        }
    });

    ui.invoke_port_changed(); // we must attempt to connect to the first available port
    ui.run()?;

    Ok(())
}
