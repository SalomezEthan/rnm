use std::{env, error::Error, fs, path::{Path}, process::exit};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();

    let Some(path_entry) = args.nth(1) else {
        eprintln!("use : rnm <file path> <name>");
        exit(2)
    };

    let Some(new_name) = args.next() else {
        eprintln!("use : rnm <file path> <name>");
        exit(2);
    };

    let path = Path::new(&path_entry);
    if path.exists() {
        
        let mut new_path = String::new();
        
        if let Some(parent) = path.parent() {
            new_path.push_str(parent.to_str().unwrap());
            new_path.push('/');
        }

        new_path.push_str(new_name.as_str());
        
        if let Some(ext) = path.extension() && !new_name.ends_with(ext.to_str().unwrap()) {
            new_path.push('.');
            new_path.push_str(ext.to_str().unwrap());
        }

        fs::rename(path_entry, new_path)?;
        return Ok(())
    }
    else {
        eprintln!("Le chemin n'existe pas !");
        exit(2);
    }
}
