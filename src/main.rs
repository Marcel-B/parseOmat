use std::env;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Read;
use std::io::Write;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1
    {
        return;
    }
    let path = Path::new(&args[1]);
    let display = path.display();
    let mut file = match File::open(&path){
        Err(why) => panic!("Fehler beim öffnen von {}: {}", display, why.description()), 
        Ok(file) => file,
    };

    let mut current_table = String::new();
    match file.read_to_string(&mut current_table){
        Err(why) => panic!("Fehler beim einlesen von {}: {}", display, why.description()),
        Ok(_) => print!("{}", display),
    };

    current_table = current_table.replace(",", ".");
    current_table = current_table.replace(";", ",");

    let file_copy_path = Path::new("copy.csv");
    let copy_display = file_copy_path.display();
    let mut copy_file = match File::create(&file_copy_path){
        Err(why) => panic!("Konnte {} erstellen: {}", copy_display, why.description()),
        Ok(copy_file) => copy_file,
    };
    match copy_file.write_all(current_table.as_bytes()){
        Err(why) => panic!("Schreiben von {} fehlgeschlagen: {}", copy_display, why.description()),
        Ok(_) => println!("Erfolgreich zu {} geschrieben.", copy_display),
    };

}