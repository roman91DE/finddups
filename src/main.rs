use finddups::config::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <directory> [--delete]", args[0]);
        std::process::exit(1);
    }
    let root_dir = std::path::PathBuf::from(&args[1]);
    let delete = args.iter().any(|a| a == "--delete");
    let config = Config::new(root_dir, delete);
    let dups = finddups::dups::find_duplicates(&config.root_dir);
    if dups.is_empty() {
        println!("No duplicates found.");
        return;
    }
    for (_hash, files) in &dups {
        if files.len() > 1 {
            println!("Duplicate group:");
            for file in files {
                println!("  {}", file.display());
            }
            if config.delete {
                if let Err(e) = finddups::dups::delete_files(files) {
                    eprintln!("Error deleting files: {}", e);
                } else {
                    println!("Deleted above files.");
                }
            }
        }
    }
}
