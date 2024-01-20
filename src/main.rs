use std::{ffi::CString, path::Path, time::Duration};

use winapi::um::winbase::MoveFileExA;


const FLAG: u32 = 0x00000004;

fn read(prompt: &str) -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Error reading input");
    s.trim().to_string()
}

#[allow(non_snake_case)]
unsafe fn MoveFileEx(filepath: &CString) {
    MoveFileExA(filepath.as_bytes_with_nul().as_ptr() as _, std::ptr::null_mut(), FLAG);
}

fn delete_file_regular(file_folder: &Path) {
    if file_folder.exists() {
        if file_folder.is_dir() {
            if std::fs::read_dir(file_folder).expect("Error reading directory.").count() > 0 {
                std::fs::remove_dir_all(file_folder).expect("Error removing directory and its contents.");
                println!("Folder at {} removed along with its contents.", file_folder.display());
            } else {
                std::fs::remove_dir(file_folder).expect("Error removing empty directory.");
                println!("Empty folder at {} removed.", file_folder.display());
            }
        } else {
            std::fs::remove_file(file_folder).expect("Error removing file.");
            println!("File at {} removed.", file_folder.display());
        }
    } else {
        println!("{} does not exist", file_folder.to_str().unwrap());
    }
    std::thread::sleep(Duration::from_secs(3));
}

fn handle_file_folder_moveex(file_folder: &Path) {
    if file_folder.exists() {
        let file = CString::new(file_folder.to_str().unwrap()).expect("Error creating CStr");
        if file_folder.is_dir() {
            println!("Handling folder: {}", file_folder.display());
            for once in file_folder.read_dir().expect("Error reading dir.") {
                match once {
                    Ok(x) => {
                        handle_file_folder_moveex(x.path().as_path());
                    },
                    Err(_) => {
                        println!("Error reading DirEntry");
                    }
                }
            }
            unsafe {
                MoveFileEx(&file);
            }
        } else {
            unsafe {
                MoveFileEx(&file);
            }
            println!("File at {} delete on boot...", file.to_str().unwrap());
        }
    } else {
        println!("{} does not exist", file_folder.display());
    }
    std::thread::sleep(Duration::from_secs(3));
}

fn main_menu() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("File removal options:");
    println!("[1] Regular OS File removal. (Might need admin rights)");
    println!("[2] File removal using MoveFileEx when you reboot. (Needs admin rights)");
    let user_input = read("Option: ");
    match user_input.parse::<u32>() {
        Ok(value) => {
            if value > 2 {
                println!("Invalid Input. Input is bigger than expected.");
                std::thread::sleep(Duration::from_secs(2));
                main_menu();
            } else {
                let path_input = read("File/Folder path: ");
                let path = std::path::Path::new((&path_input).as_str());
                match value {
                    1 => {
                        delete_file_regular(path);
                    },
                    2 => {
                        handle_file_folder_moveex(path);
                    },
                    _ => {
                        println!("Invalid Input.");
                        std::thread::sleep(Duration::from_secs(2));
                        main_menu();
                    }
                }
            }
        },
        Err(_) => {
            println!("Invalid Input. Input is not a number or a negative one.");
            std::thread::sleep(Duration::from_secs(2));
            main_menu();
        } 
    }
    main_menu();
}

fn main() {
    main_menu();
}