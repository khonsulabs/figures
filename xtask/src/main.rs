use khonsu_tools::{anyhow, code_coverage::CodeCoverage};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Args {
    GenerateCodeCoverageReport {
        #[structopt(long = "install-dependencies")]
        install_dependencies: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::from_args();
    match args {
        Args::GenerateCodeCoverageReport {
            install_dependencies,
        } => CodeCoverage::<CodeCoverageConfig>::execute(install_dependencies)?,
    };
    Ok(())
}

struct CodeCoverageConfig;

impl khonsu_tools::code_coverage::Config for CodeCoverageConfig {
    fn ignore_paths() -> Vec<String> {
        vec![String::from("figures/examples/*")]
    }
}
