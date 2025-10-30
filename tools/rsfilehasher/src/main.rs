//! Ett verktyg för att beräkna kryptografiska hashsummor för filer.
//!
//! Stödjer MD5, SHA-1 och SHA-256 algoritmer.
//!
//! Exempel:
//! ```
//! filehash -f dokument.txt --hash md5
//! filehash -d /path/to/dir --hash sha256
//! filehash --dir ./myfolder --hash sha1
//! ``` 
use clap::{Arg, ArgAction, Command};
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Digest};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process;

const VERSION: &str = "1.0.0";
const AUTHOR: &str = "File Hash Calculator";

struct Config {
    file: Option<String>,
    dir: Option<String>,
    hash_type: String,
    about: bool,
}

struct FileInfo {
    name: String,
    size: u64,
    hash: String,
}

fn main() {
    let config = parse_args();
    
    if config.about {
        print_about();
        return;
    }
    
    // Validera hash-typ
    if !["md5", "sha1", "sha256"].contains(&config.hash_type.as_str()) {
        eprintln!("Fel: Ogiltig hash-typ '{}'. Använd md5, sha1 eller sha256.", config.hash_type);
        process::exit(1);
    }
    
    // Kontrollera att antingen -f eller -d är angiven
    if config.file.is_none() && config.dir.is_none() {
        eprintln!("Fel: Du måste ange antingen -f/--file eller -d/--dir");
        eprintln!("Använd --help för mer information");
        process::exit(1);
    }
    
    // Kontrollera att inte både -f och -d är angivna
    if config.file.is_some() && config.dir.is_some() {
        eprintln!("Fel: Du kan inte använda både -f/--file och -d/--dir samtidigt");
        process::exit(1);
    }
    
    let files = if let Some(file_path) = config.file {
        // Verifiera att filen existerar
        if !Path::new(&file_path).exists() {
            eprintln!("Fel: Filen '{}' finns inte", file_path);
            process::exit(1);
        }
        vec![PathBuf::from(file_path)]
    } else if let Some(dir_path) = config.dir {
        match get_files_in_dir(&dir_path) {
            Ok(files) => {
                if files.is_empty() {
                    eprintln!("Inga filer hittades i katalogen '{}'", dir_path);
                    process::exit(1);
                }
                files
            }
            Err(e) => {
                eprintln!("Fel vid läsning av katalog: {}", e);
                process::exit(1);
            }
        }
    } else {
        vec![]
    };
    
    // Beräkna hash och skriv ut tabell
    print_hash_table(files, &config.hash_type);
}

fn parse_args() -> Config {
    let matches = Command::new("filehash")
        .version(VERSION)
        .author(AUTHOR)
        .about("Beräknar hashsummor för filer och visar resultatet i en tabell.")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FIL")
                .help("Ange fil att hasha")
                .conflicts_with("dir")
        )
        .arg(
            Arg::new("dir")
                .short('d')
                .long("dir")
                .value_name("KATALOG")
                .help("Ange katalog för att hasha alla filer")
                .conflicts_with("file")
        )
        .arg(
            Arg::new("hash")
                .long("hash")
                .value_name("TYP")
                .help("Hash-algoritm: md5, sha1, sha256 (standard: sha256)")
                .default_value("sha256")
        )
        .arg(
            Arg::new("about")
                .long("about")
                .help("Visa information om programmet")
                .action(ArgAction::SetTrue)
        )
        .after_help(
            "Exempel:\n  \
            filehash -f dokument.txt --hash md5\n  \
            filehash -d /path/to/dir --hash sha256\n  \
            filehash --dir ./myfolder --hash sha1"
        )
        .get_matches();
    
    Config {
        file: matches.get_one::<String>("file").cloned(),
        dir: matches.get_one::<String>("dir").cloned(),
        hash_type: matches.get_one::<String>("hash").unwrap().to_string(),
        about: matches.get_flag("about"),
    }
}

fn print_about() {
    println!("{} v{}", AUTHOR, VERSION);
    println!("Ett verktyg för att beräkna kryptografiska hashsummor för filer.");
    println!("Stödjer MD5, SHA-1 och SHA-256 algoritmer.");
    println!("\nAnvändning:");
    println!("- Verifiera filintegritet");
    println!("- Kontrollera att filer inte har ändrats");
    println!("- Skapa checksummor för säkerhetskopiering");
}

fn get_files_in_dir(dir: &str) -> io::Result<Vec<PathBuf>> {
    let path = Path::new(dir);
    
    // Kontrollera att katalogen existerar
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Katalogen '{}' finns inte", dir)
        ));
    }
    
    if !path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("'{}' är inte en katalog", dir)
        ));
    }
    
    let mut files = Vec::new();
    
    // Läs alla filer i katalogen (inte rekursivt)
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            files.push(path);
        }
    }
    
    Ok(files)
}

fn calculate_hash(file_path: &Path, hash_type: &str) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    let hash_result = match hash_type {
        "md5" => {
            let mut hasher = Md5::new();
            hasher.update(&buffer);
            format!("{:x}", hasher.finalize())
        }
        "sha1" => {
            let mut hasher = Sha1::new();
            hasher.update(&buffer);
            format!("{:x}", hasher.finalize())
        }
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(&buffer);
            format!("{:x}", hasher.finalize())
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Okänd hash-typ: {}", hash_type)
            ));
        }
    };
    
    Ok(hash_result)
}

fn get_file_size(file_path: &Path) -> io::Result<u64> {
    let metadata = fs::metadata(file_path)?;
    Ok(metadata.len())
}

fn format_size(size: u64) -> String {
    const UNIT: u64 = 1024;
    
    if size < UNIT {
        return format!("{} B", size);
    }
    
    let units = ['K', 'M', 'G', 'T', 'P', 'E'];
    let mut div = UNIT;
    let mut exp = 0;
    
    let mut n = size / UNIT;
    while n >= UNIT && exp < units.len() - 1 {
        div *= UNIT;
        exp += 1;
        n /= UNIT;
    }
    
    format!("{:.1} {}B", size as f64 / div as f64, units[exp])
}

fn print_hash_table(files: Vec<PathBuf>, hash_type: &str) {
    let mut file_infos = Vec::new();
    
    // Samla all information först
    for file_path in files {
        match calculate_hash(&file_path, hash_type) {
            Ok(hash) => {
                match get_file_size(&file_path) {
                    Ok(size) => {
                        let name = file_path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("?")
                            .to_string();
                        
                        file_infos.push(FileInfo { name, size, hash });
                    }
                    Err(e) => {
                        eprintln!("Fel vid hämtning av filstorlek för {:?}: {}", file_path, e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Fel vid beräkning av hash för {:?}: {}", file_path, e);
            }
        }
    }
    
    // Beräkna kolumnbredder för snygg formatering
    let max_name_len = file_infos
        .iter()
        .map(|f| f.name.len())
        .max()
        .unwrap_or(7)
        .max(7); // Minst lika bred som "Filnamn"
    
    let max_size_len = file_infos
        .iter()
        .map(|f| format_size(f.size).len())
        .max()
        .unwrap_or(7)
        .max(7); // Minst lika bred som "Storlek"
    
    // Skriv header
    println!(
        "{:<width_name$}  {:<width_size$}  {} Hash",
        "Filnamn",
        "Storlek",
        hash_type,
        width_name = max_name_len,
        width_size = max_size_len
    );
    
    println!(
        "{:<width_name$}  {:<width_size$}  {}",
        "-".repeat(7),
        "-".repeat(7),
        "----",
        width_name = max_name_len,
        width_size = max_size_len
    );
    
    // Skriv filrader
    for info in file_infos {
        println!(
            "{:<width_name$}  {:<width_size$}  {}",
            info.name,
            format_size(info.size),
            info.hash,
            width_name = max_name_len,
            width_size = max_size_len
        );
    }
}