fn main() {
    match starcompass::run(std::env::args_os().skip(1)) {
        Ok(output) => println!("{output}"),
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(1);
        }
    }
}
