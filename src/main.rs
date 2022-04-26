use anyhow::{anyhow, Result};
use krew_wasm_plugin_sdk::kube_conf::Config;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

mod cli;
use clap::Parser;

mod print_cert;
mod secret;

fn main() {
    let cli = cli::Cli::parse();

    // setup logging
    let level_filter = if cli.verbose { "debug" } else { "info" };
    let filter_layer = EnvFilter::new(level_filter).add_directive("hyper=off".parse().unwrap()); // this crate generates lots of tracing events we don't care about
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt::layer().with_writer(std::io::stderr))
        .init();

    if let Err(e) = run(cli) {
        eprint!("{}", e);
        std::process::exit(1);
    }
}

fn run(cli: cli::Cli) -> Result<()> {
    let kube_config = Config::load_default().map_err(|e| anyhow!("{:?}", e))?;
    let current_context = kube_config
        .get_current_context()
        .ok_or_else(|| anyhow!("Cannot find default kubeconfig context"))?;
    let default_namespace = current_context
        .namespace
        .clone()
        .unwrap_or_else(|| "default".to_string());

    match cli.command {
        cli::Commands::Secret {
            namespace,
            disable_cert_decoding,
            name,
        } => secret::decode(
            &name,
            namespace.unwrap_or(default_namespace).as_str(),
            disable_cert_decoding,
        ),
    }
}
