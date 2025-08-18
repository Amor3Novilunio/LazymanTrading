// -------------
// Custom Macros
// -------------

#[macro_export]
macro_rules! std_error_exit {
    ($message:expr) => {{
        eprintln!("{}", $message);
        ::std::process::exit(1);
    }};
}

#[macro_export]
macro_rules! dprintln {
    ($message:expr) => {{
        println!("{:?}", $message);
    }};
}

#[macro_export]
macro_rules! clear_terminal {
    () => {
        let panic_resp = "Failed to Execute Clear Command";
        // Check if the program is running on Windows
        if cfg!(target_os = "windows") {
            // On Windows, use the `cls` command, but it must be run through `cmd`
            std::process::Command::new("cmd")
                .args(&["/C", "cls"])
                .status()
                .expect(panic_resp);
        } else {
            // On Unix-like systems (Linux, macOS), use the `clear` command
            std::process::Command::new("clear").status().expect(panic_resp);
        }
    };
}