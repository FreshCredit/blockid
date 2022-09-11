use sc_cli::RunCmd;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
  #[structopt(flatten)]
  pub run: RunCmd,

  #[structopt(subcommand)]
  pub subcommand: Option<Subcommand>,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
  #[structopt(name = "benchmark", about = "Benchmark runtime pallets.")]
  Benchmark(frame_benchmarking_cli::BenchmarkCmd),
  BuildSpec(sc_cli::BuildSpecCmd),
  CheckBlock(sc_cli::CheckBlockCmd),
  ExportBlocks(sc_cli::ExportBlocksCmd),
  ExportState(sc_cli::ExportStateCmd),
  ImportBlocks(sc_cli::ImportBlocksCmd),
  Key(sc_cli::KeySubcommand),
  PurgeChain(sc_cli::PurgeChainCmd),
  Revert(sc_cli::RevertCmd),
  Metadata,
}
