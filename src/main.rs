use std::{ffi::CString, path::Path};

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
unsafe fn MoveFileEx(filepath: CString) {
    MoveFileExA(filepath.as_bytes_with_nul().as_ptr() as _, std::ptr::null_mut(), FLAG);
}

fn handle_file_folder(file_folder: &Path) {
    if file_folder.exists() {
        let file = CString::new(file_folder.to_str().unwrap()).expect("Error creating CStr");
        if file_folder.is_dir() {
            println!("Handling folder: {}", file_folder.to_str().unwrap());
            for once in file_folder.read_dir().expect("Error reading dir.") {
                match once {
                    Ok(x) => {
                        handle_file_folder(x.path().as_path());
                    },
                    Err(_) => {
                        println!("Error reading DirEntry");
                    }
                }
            }
            unsafe {
                MoveFileEx(file.clone());
            }
        } else {
            unsafe {
                MoveFileEx(file.clone());
            }
            println!("File at {} delete on boot...", file.to_str().unwrap());
        }
    } else {
        println!("{} does not exist", file_folder.to_str().unwrap());
    }
}

fn main() {
    let inp = read("File path: ");
    let p = std::path::Path::new((&inp).as_str());
    handle_file_folder(p);
    read("Press any key to exit...");
}
