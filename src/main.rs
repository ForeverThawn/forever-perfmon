#[cfg(windows)]
mod app;

#[cfg(windows)]
fn main() -> std::io::Result<()> {
    app::run()
}

#[cfg(not(windows))]
fn main() {
    eprintln!("forever-perfmon currently targets Windows performance counters.");
}
