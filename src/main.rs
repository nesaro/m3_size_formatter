use std::env;

#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

impl FileSize {
    fn from_bytes(size: u64) -> Self
    {
        match size {
            0..=999 => FileSize::Bytes(size),
            1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
            1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
            _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
        }

    }
}

fn format_size(size: u64) -> String {
    let filesize = FileSize::from_bytes(size);
    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let result = format_size(6888837399);
    let size = args[1].parse::<f64>().unwrap();
    let unit = &args[2];
    let actual_size = match unit.as_str() {
        "b" => FileSize::Bytes(size as u64),
        "kb" => FileSize::Kilobytes(size),
        "mb" => FileSize::Megabytes(size),
        "gb" => FileSize::Gigabytes(size),

        _ => panic!("unknown input"),
    };
    println!("{}", result);
    println!("{:?}", actual_size);
}
