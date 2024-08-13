use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use chrono::{DateTime, Local};
use prettytable::{Table, row};

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    match bytes {
        _ if bytes >= GB => format!("{:.2} GB", bytes as f64 / GB as f64),
        _ if bytes >= MB => format!("{:.2} MB", bytes as f64 / MB as f64),
        _ if bytes >= KB => format!("{:.2} KB", bytes as f64 / KB as f64),
        _ => format!("{} bytes", bytes),
    }
}

fn main() -> io::Result<()> {
    let paths = fs::read_dir(".")?;
    let mut table = Table::new();

    table.add_row(row!["Name", "Permission", "Date of modification", "Size"]);

    for path in paths {
        let path = path?.path();

        let metadata = fs::metadata(&path)?;
        let permission = metadata.permissions().mode();
        let modified = metadata.modified()?;
        let file_size = metadata.len();
        let formatted_size = format_size(file_size);

        let datetime = DateTime::<Local>::from(modified);
        let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        table.add_row(row![path.display(), permission, formatted_time, formatted_size]);
    }

    table.printstd();

    Ok(())
}
