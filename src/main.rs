use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("\n\x1b[1m🌋 welcome to lava, the obsidian vault duplicator! 🌋\x1b[0m\n");

    // Get vault path from arguments or prompt
    let args: Vec<String> = env::args().collect();
    let mut vault_path_str = String::new();

    if args.len() > 1 {
        vault_path_str = args[1..].join(" ");
    } else {
        print!("\x1b[1mdrag and drop your obsidian vault folder here and press enter:\x1b[0m\n> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut vault_path_str)
            .expect("Failed to read input");
    }

    // Clean up the path string (macOS terminal drag-and-drop often adds quotes or trailing spaces)
    let vault_path_str = vault_path_str.trim().trim_matches('\'').trim_matches('"');
    let vault_path = Path::new(vault_path_str);

    if !vault_path.exists() || !vault_path.is_dir() {
        eprintln!("Error: The path provided is not a valid directory.");
        return;
    }

    let vault_name = vault_path.file_name().unwrap_or_default().to_string_lossy();
    let parent_dir = vault_path.parent().unwrap_or_else(|| Path::new("."));

    println!("\n\x1b[1mvault detected:\x1b[0m {}\n", vault_name);
    println!("\x1b[1mwhat type of duplicate would you like to make?\x1b[0m");
    println!("  \x1b[1m1)\x1b[0m full duplicate (copies everything: notes, files, settings)");
    println!("  \x1b[1m2)\x1b[0m template duplicate (copies ONLY extensions, themes, and data (.obsidian), leaves notes behind)\n");
    print!("\x1b[1mchoose an option (1 or 2):\x1b[0m ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    let choice = choice.trim();

    match choice {
        "1" => {
            let new_vault_name = format!("{} duplicate", vault_name);
            let dest_path = parent_dir.join(&new_vault_name);

            println!("copying entire vault to: {}...", dest_path.display());

            let status = Command::new("cp")
                .arg("-a")
                .arg(vault_path)
                .arg(&dest_path)
                .status()
                .expect("failed to execute cp command");

            if status.success() {
                println!("\n\x1b[1m✅ full duplicate created successfully!\x1b[0m\n");
            } else {
                eprintln!("\n\x1b[1m❌ failed to create full duplicate.\x1b[0m\n");
            }
        }
        "2" => {
            print!("\x1b[1menter the name for the new vault:\x1b[0m ");
            io::stdout().flush().unwrap();
            let mut new_vault_name = String::new();
            io::stdin()
                .read_line(&mut new_vault_name)
                .expect("failed to read input");
            let new_vault_name = new_vault_name.trim();

            if new_vault_name.is_empty() {
                eprintln!("error: name cannot be empty.");
                return;
            }

            let dest_path = parent_dir.join(new_vault_name);

            // Create the new vault directory
            if let Err(e) = fs::create_dir_all(&dest_path) {
                eprintln!("Error creating new vault directory: {}", e);
                return;
            }

            let obsidian_folder_src = vault_path.join(".obsidian");
            let obsidian_folder_dest = dest_path.join(".obsidian");

            if !obsidian_folder_src.exists() {
                println!("warning: original vault does not have a .obsidian folder.");
                println!("empty vault '{}' created successfully!", new_vault_name);
                return;
            }

            println!(
                "copying .obsidian configuration to: {}...",
                dest_path.display()
            );

            let status = Command::new("cp")
                .arg("-a")
                .arg(&obsidian_folder_src)
                .arg(&obsidian_folder_dest)
                .status()
                .expect("failed to execute cp command");

            if status.success() {
                println!("\n\x1b[1m✅ template duplicate created successfully!\x1b[0m\n");
            } else {
                eprintln!("\n\x1b[1m❌ failed to copy .obsidian configuration.\x1b[0m\n");
            }
        }
        _ => {
            eprintln!("invalid choice. Please run the program again and select 1 or 2.");
        }
    }
}
