//use std::os::unix::fs;
use std::fs;
use std::os::unix::fs as unix_fs;

pub fn ln(arguments: &[String]) {
    let mut symbolic = false;
    let mut start_index = 0;

    if let Some(arg) = arguments.get(0) {
        if arg == "-s" || arg == "--symbolic" {
            symbolic = true;
            start_index = 1;
        }
    }

    let src = match arguments.get(start_index) {
        Some(name) => name,
        None => {
            eprintln!("ln: lipsește fișierul sursă");
            return;
        }
    };

    let dst = match arguments.get(start_index + 1) {
        Some(name) => name,
        None => {
            eprintln!("ln: lipsește fișierul destinație");
            return;
        }
    };

    if symbolic {
        match unix_fs::symlink(src, dst) {
            Ok(_) => {}
            Err(e) => eprintln!("ln: nu am putut crea link-ul simbolic '{}': {}", dst, e),
        }
    } else {
        match fs::hard_link(src, dst) {
            Ok(_) => {}
            Err(e) => eprintln!("ln: nu am putut crea hard link-ul '{}': {}", dst, e),
        }
    }
}
