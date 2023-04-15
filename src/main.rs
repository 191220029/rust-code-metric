use clap::Parser;
use code_metric::Collector;
use command::Cli;
use fs::{iter_directory_recursively, iter_benchmark_suit};

mod fs;
mod code_metric;
mod command;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        command::Commands::MetricDefault { target, out_dir } => {
            let mut collector = Collector::new();
            match iter_directory_recursively(&target, &mut collector) {
                Ok(_) => collector.to_xls_file(&out_dir.join("code_metric.xlsx")),
                Err(e) => eprintln!("{}", e),
            }
        }
        command::Commands::MetricBenchmarkSuit { target, out_dir } => {
            match iter_benchmark_suit(&target, &out_dir.join("code_metric.xlsx")) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e),
            }
        },
    }
}
