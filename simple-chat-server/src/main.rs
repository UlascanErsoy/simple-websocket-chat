pub mod config;
pub mod server;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: Option<String>
}

fn main() {
    
    let args = Args::parse();
    
    let config_path = match &args.config {
        Some(path) => path,
        None => "config.yaml"
    };
    
    let config = config::ServerConfig::from_file(config_path);
    
    let server = server::ChatServer::new(config);

    server.bind().run();
    
}
