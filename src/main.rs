use std::fs;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    from: String,
    to: String,
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    println!("from {} into {}", args.from, args.to);

    for entry in fs::read_dir(".")? {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Some(pathname) = entry.path().to_str() {
                        if pathname.contains(&args.from) {
                            fs::rename(pathname, pathname.replace(&args.from, &args.to))?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
