# retrieve_a_file_from_a_hard_disk-


File Reading Examples

This repository contains examples of how to read a file from the disk using two different programming languages: Python and Rust. These examples are designed to demonstrate basic file reading operations, including error handling for cases where the file does not exist.
Python Example
Description

The Python script (read_file.py) demonstrates how to check if a file exists and read its content. It uses the os module to ensure compatibility across different operating systems.
Usage

To use the Python script, replace the chemin/vers/ton/fichier.txt with the actual path to your file. You can run the script using a Python interpreter:

sh

python read_file.py

Code

python

import os

chemin_du_fichier = 'chemin/vers/ton/fichier.txt'

if os.path.exists(chemin_du_fichier):
    with open(chemin_du_fichier, 'r') as fichier:
        contenu = fichier.read()
        print(contenu)
else:
    print("Le fichier n'existe pas")

Rust Example
Description

The Rust program (src/main.rs) showcases how to perform file reading with proper error handling. It utilizes Rust's strong type system and error handling features to manage possible errors, such as non-existent files.
Usage

To use the Rust example, first replace the chemin/vers/ton/fichier.txt with the path to your file. Compile and run the program with Cargo, Rust's package manager and build system:

sh

cargo run

Code

rust

use std::fs;
use std::io;
use std::path::Path;

fn lire_fichier(chemin: &str) -> Result<String, io::Error> {
    if Path::new(chemin).exists() {
        fs::read_to_string(chemin)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Le fichier n'existe pas"))
    }
}

fn main() {
    let chemin_du_fichier = "chemin/vers/ton/fichier.txt";
    match lire_fichier(chemin_du_fichier) {
        Ok(contenu) => println!("{}", contenu),
        Err(e) => println!("Erreur lors de la lecture du fichier: {}", e),
    }
}

Contributing

Contributions are welcome! If you have improvements or bug fixes, please open a pull request or issue.
License

This project is licensed under the MIT License - see the LICENSE.md file for details.
