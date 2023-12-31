#[macro_export]
macro_rules! input_path {
    () => {
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/input"))
    };
}

#[macro_export]
macro_rules! input_file {
    () => {
        std::fs::File::open($crate::input_path!())
    };
}

#[macro_export]
macro_rules! input_lines {
    () => {
        $crate::input_file!().map(|f| std::io::BufRead::lines(std::io::BufReader::new(f)))
    };
}
