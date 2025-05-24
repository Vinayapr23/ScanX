use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Target domain or IP to scan
    #[arg(short = 't', long)]
    pub target: String,

    /// Start port number (inclusive)
    #[arg(short = 's', long, default_value_t = 1)]
    pub start: u16,

    /// End port number (inclusive)
    #[arg(short = 'e', long, default_value_t = 1024)]
    pub end: u16,

    /// Output format: json or html or table
    #[arg(short = 'f', long, default_value = "table")]
    pub output: String,

    /// Timeout per port scan in milliseconds
    #[arg(short = 'o', long, default_value_t = 3000)]
    pub timeout: u64,

    /// Enable banner grabbing
    #[arg(long, default_value_t = false)]
    pub banner: bool,
}
