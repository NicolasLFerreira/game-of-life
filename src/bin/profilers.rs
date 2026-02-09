#[cfg(unix)]
fn main() {
    let guard = pprof::ProfilerGuardBuilder::default()
        .frequency(10000)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build()
        .unwrap();

    let soup = thanatos::types::CellConfiguration::cook_soup(42, 10, 10, 0.2);
    let cconf = thanatos::types::CellConfiguration::from_soup(soup);
    for _ in 0..10_000 {
        thanatos::conway::step(&cconf);
    }

    if let Ok(report) = guard.report().build() {
        println!("Saving .svg");
        let file = std::fs::File::create("tracing.svg").unwrap();
        report.flamegraph(file).unwrap();
    }
}

#[cfg(windows)]
fn main() {
    // screw them W*ndows users
    eprintln!(
        "Profiling support only for UNIX-based systems, '{}' is not supported.",
        std::env::consts::OS
    );
    std::process::exit(0);
}
