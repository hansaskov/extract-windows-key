# Windows Key Retriever

A simple Rust program that retrieves the Windows product key from your system, displays it in a GUI, and allows you to save it to a CSV file.

![Example of the user interface](public/example-gui.png "User Interface")

## Dependencies

This program uses the following crates:

- native-windows-gui
- std

## Usage

1. Compile the program with `cargo build --release`.
2. Run the compiled executable located in `./target/release/` directory.
3. The Windows key will be displayed in the GUI.
4. Click the "Save" button to save the Windows key and your computer's name to a CSV file named `windows_keys.txt`.
5. Click the "Close" button to close the program.

## Functionality

The program has the following main functions:

- `main`: Initializes the GUI and sets up the event handler for button clicks.
- `get_computer_name`: Retrieves the computer's name.
- `get_windows_key`: Retrieves the Windows product key using a PowerShell command.
- `save_windows_key_to_csv`: Saves the Windows key and computer name to a CSV file.

## License

This project is released under the [MIT License](https://opensource.org/licenses/MIT).
