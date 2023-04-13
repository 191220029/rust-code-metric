use clap::Parser;
use code_metric::Collector;
use command::Cli;
use fs::iter_directory_recursively;

mod fs;
mod code_metric;
mod command;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        command::Commands::MetricDefault { target, out_dir } => {
            let mut collector = Collector::new();
            match iter_directory_recursively(&target, &mut collector) {
                Ok(_) => collector.to_xls(&out_dir.join("code_metric.xls")),
                Err(e) => eprintln!("{}", e),
            }
            
        }
    }
}
