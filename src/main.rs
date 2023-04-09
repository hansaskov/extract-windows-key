#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
use std::fs::OpenOptions;
use std::env;
use std::io::{Error as IOError, Write};
use std::path::Path;
use std::process::Command;

use std::rc::Rc;

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let mut window = Default::default();
    let mut key_label = Default::default();
    let mut key_text = Default::default();
    let mut close_button = Default::default();
    let mut save_button = Default::default(); // Add save_button
    let layout = Default::default();

    nwg::Window::builder()
        .size((400, 150))
        .position((300, 300))
        .title("Windows Key")
        .build(&mut window)
        .unwrap();

    let windows_key = get_windows_key();

    nwg::Label::builder()
        .text("Your Windows key is:")
        .parent(&window)
        .build(&mut key_label)
        .unwrap();

    nwg::TextInput::builder()
        .text(&windows_key.clone().unwrap_or_default())
        .parent(&window)
        .readonly(true)
        .build(&mut key_text)
        .unwrap();

    nwg::Button::builder()
        .text("Close")
        .parent(&window)
        .build(&mut close_button)
        .unwrap();

    nwg::Button::builder() // Add save_button builder
        .text("Save")
        .parent(&window)
        .build(&mut save_button)
        .unwrap();

    nwg::GridLayout::builder()
        .parent(&window)
        .spacing(1)
        .child(0, 0, &key_label)
        .child(1, 0, &key_text)
        .child(0, 1, &close_button)
        .child(1, 1, &save_button) // Include save_button in layout
        .build(&layout)
        .unwrap();

    let window = Rc::new(window);
    let events_window = window.clone();

    let handler = nwg::full_bind_event_handler(&window.handle, move |evt, _evt_data, handle| {
        use nwg::Event as E;

        match evt {
            E::OnWindowClose => {
                if &handle == &events_window as &nwg::Window {
                    nwg::stop_thread_dispatch();
                }
            }
            E::OnButtonClick => {
                if &handle == &close_button {
                    nwg::stop_thread_dispatch();
                } else if &handle == &save_button {
                    let computer_name = match get_computer_name() {
                        Ok(name) => name,
                        Err(e) => {
                            eprintln!("Error getting computer name: {}", e);
                            return;
                        },
                    };
            
                    if let Err(e) = save_windows_key_to_csv(&computer_name, &windows_key.clone()) {
                        eprintln!("Error saving Windows key and computer name to CSV file: {}", e);
                    } else {
                        println!("Windows key and computer name saved to CSV file");
                    }
                }
            }
            _ => {}
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}

fn get_computer_name() -> Result<String, IOError> {
    match env::var("COMPUTERNAME") {
        Ok(name) => Ok(name),
        Err(_) => Err(IOError::new(std::io::ErrorKind::NotFound, "Computer name not found.")),
    }
}

fn get_windows_key() -> Option<String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    let output = Command::new("powershell")
        .arg("-Command")
        .arg("(Get-WmiObject -query 'select * from SoftwareLicensingService').OA3xOriginalProductKey")
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .expect("Failed to execute PowerShell command");

    if output.status.success() {
        let key = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !key.is_empty() {
            return Some(key);
        }
    }

    None
}

fn save_windows_key_to_csv(computer_name: &str, windows_key: &Option<String>) -> Result<(), IOError> {
    if let Some(key) = windows_key {
        let file_path = Path::new("windows_keys.txt");
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_path)?;
        file.write_all(format!("{}, {}\n", computer_name, key).as_bytes())?;
    } else {
        eprintln!("No Windows key to save.");
        return Err(IOError::new(std::io::ErrorKind::InvalidData, "No Windows key to save."));
    }

    Ok(())
}