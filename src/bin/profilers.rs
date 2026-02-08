use std::fs::File;
use std::io::Write;
use thanatos::types::cell_configuration::CellConfiguration;
use thanatos::*;

fn main() {
    let seed = CellConfiguration::random_configuration(42, 10, 10, 0.2);
    let cconf = CellConfiguration::with_seed_configuration(seed);

    let guard = pprof2::ProfilerGuardBuilder::default()
        .frequency(10000)
        .build()
        .unwrap();

    for _ in 0..10_000 {
        conway::step(&cconf);
    }

    if let Ok(report) = guard.report().build() {
        // Flamegraph SVG
        let file = File::create("profile.svg").unwrap();
        report.flamegraph(file).unwrap();

        // CSV summary
        let mut csv = File::create("profile.csv").unwrap();
        writeln!(csv, "function,samples").unwrap();

        for (frames, count) in &report.data {
            let func_path: String = frames
                .frames
                .iter()
                .map(|stack| {
                    stack
                        .iter()
                        .map(|s| {
                            s.name
                                .as_ref()
                                .map(|v| String::from_utf8_lossy(v).to_string())
                                .unwrap_or("<unknown>".to_string())
                        })
                        .collect::<Vec<_>>()
                        .join(";")
                })
                .collect::<Vec<_>>()
                .join("|"); // separate multiple stacks

            writeln!(csv, "{},{}", func_path, count).unwrap();
        }
    }
}
