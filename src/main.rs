use clap::Parser;
use finddups::config::Config;

/// Find duplicate files in a directory tree
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Directory to search
    pub directory: std::path::PathBuf,
    /// Delete duplicate files
    #[arg(long)]
    pub delete: bool,
    /// Maximum directory depth (0 = unlimited)
    #[arg(long, default_value_t = 1)]
    pub depth: usize,
    /// Include hidden files and directories
    #[arg(long)]
    pub hidden: bool,
    /// Use single-threaded mode (disable parallel hashing)
    #[arg(long)]
    pub single_threaded: bool,
}

fn main() {
    let cli = Cli::parse();
    let config = Config::new(cli.directory, cli.delete, cli.depth, cli.hidden);
    let dups = finddups::dups::find_duplicates(
        &config.root_dir,
        config.max_depth,
        config.include_hidden,
        cli.single_threaded,
    );
    if dups.is_empty() {
        println!("No duplicates found.");
        return;
    }
    for files in dups.values() {
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
