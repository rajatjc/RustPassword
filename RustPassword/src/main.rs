mod passwordentry;

use crate::passwordentry::prompt;
use crate::passwordentry::read_passwords_from_file;
use crate::passwordentry::ServiceInfo;

fn clr() {
    print!("{}[2J", 27 as char);
}
fn main() {
    clr();
    println!(
        "  ____                                     _  __     __          _ _
        |  _ \\ __ _ ___ _____      _____  _ __ __| | \\ \\   / /_ _ _   _| | |_ 
        | |_) / _` / __/ __\\ \\ /\\ / / _ \\| '__/ _` |  \\ \\ / / _` | | | | | __|
        |  __/ (_| \\__ \\__ \\ \\ V  V / (_) | | | (_| |   \\ V / (_| | |_| | | |_ 
        |_|   \\__,_|___/___/ \\_/\\_/ \\___/|_|  \\__,_|    \\_/ \\__,_|\\__,_|_|\\__|'                                                      "
    );
    loop {
        println!("Password Manager Menu:");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Search");
        println!("4. Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service :"),
                    prompt("Username :"),
                    prompt("Password :"),
                );
                println!("Entry added successfully.");
                entry.write_to_file();
            }
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "Service = {}
    - Username : {} 
    - Password : {}",
                        item.service, item.username, item.password
                    );
                }
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });
                let search = prompt("Search :");
                for item in &services {
                    if item.service.as_str() == search.as_str() {
                        println!(
                            "Service = {}
    - Username : {} 
    - Password : {}",
                            item.service, item.username, item.password
                        );
                    }
                }
            }
            "4" => {
                clr();
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice."),
        }
        println!("\n\n");
    }
}