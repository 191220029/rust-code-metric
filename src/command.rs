use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands
}

#[derive(clap::Subcommand, Debug)]
#[clap(rename_all = "snake_case")]
pub enum Commands {
    MetricDefault {
        #[clap(long)]
        target: PathBuf,

        #[clap(long = "out-dir")]
        out_dir: PathBuf,
    }
} 