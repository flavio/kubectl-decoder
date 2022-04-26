use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = None,
)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub command: Commands,

    /// Enable verbose mode
    #[clap(short, long, env = "KREW_WASM_VERBOSE")]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    #[clap(arg_required_else_help = true)]
    Secret {
        /// If present, the namespace scope for this CLI request
        #[clap(short, long)]
        namespace: Option<String>,

        /// Disable the automatic decoding of x509 certificates found inside of secret
        #[clap(long)]
        disable_cert_decoding: bool,

        /// Name of the secret to decode
        name: String,
    },
}
