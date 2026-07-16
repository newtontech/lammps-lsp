use clap::Parser;
use lammps_analyser::lsp::Backend;
use tower_lsp::{LspService, Server};

#[derive(Debug, Parser)]
#[command(name = "lmp-lsp", version)]
struct Cli {}

#[tokio::main]
async fn main() {
    Cli::parse();
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
