# Rust Password Vault

![Rust Password Vault](https://github.com/rajatjc/RustPassword/raw/main/images/rust_password_vault.png)

Rust Password Vault is a secure command-line password management tool built with Rust. It offers a simple yet effective way to store and manage your passwords for various services. Utilizing JSON files for storage, it ensures the safety of your sensitive information.

## Features

- **Secure Storage**: Safely store your passwords in a JSON file.
- **Add Entries**: Easily add new password entries with service name, username, and password.
- **List Entries**: View all stored password entries in a formatted manner.
- **Search Functionality**: Quickly search for specific entries by service name.

## Dependencies

Make sure you have Rust and Cargo installed on your system to build and run the project.

## Usage

To run the program, follow these steps:

1. Clone the project from [GitHub](https://github.com/rajatjc/RustPassword).
2. Navigate to the project directory in your terminal.
3. Run `cargo build --release` to compile the project.
4. Execute the compiled binary using `./target/release/rust_password_vault`.

### Menu Options

1. **Add Entry**: Add a new password entry.
2. **List Entries**: View all stored password entries.
3. **Search**: Search for entries by service name.
4. **Quit**: Exit the program.

## Screenshots

1. Menu
   ![Menu](https://github.com/rajatjc/RustPassword/blob/main/raw/main/images/1.png)

2. List Entries
   ![List Entries](https://github.com/rajatjc/RustPassword/blob/main/raw/main/images/2.png)

3. Failed Search
   ![Failed Search](https://github.com/rajatjc/RustPassword/blob/main/raw/main/images/3.png)

4. Search Found
   ![Search Found](https://github.com/rajatjc/RustPassword/blob/main/raw/main/images/4.png)

5. Goodbye
   ![Goodbye](https://github.com/rajatjc/RustPassword/blob/main/raw/main/images/5.png)

## File Structure

- `main.rs`: Contains the main program logic and the main loop.
- `pentry.rs`: Defines the `ServiceInfo` struct and its methods for handling password entries.

## Contributing

Contributions are welcome! Feel free to submit pull requests or open issues on the [project's repository](https://github.com/rajatjc/RustPassword).
