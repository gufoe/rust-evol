use std::path::PathBuf;

use render::Render;
use server::Server;
use structopt::StructOpt;

mod actions;
mod genome;
mod input;
mod net;
mod pool;
mod render;
mod replicant;
mod rng;
mod server;
mod simulation;
mod world;

/// A fictional versioning CLI
#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]

struct Cli {
    #[structopt(short, long)]
    render: bool,
    file: Option<PathBuf>,
}

fn main() {
    let args = Cli::from_args();

    let mut server: Server =
        if let Ok(content) = std::fs::read(&args.file.clone().unwrap_or("xxx".into())) {
            let server: Server = bincode::deserialize(&content).unwrap();
            server
        } else {
            let server = Server::default();
            server
        };

    server.auto_save = args.file;

    if args.render {
        Render::new(server);
    } else {
        loop {
            server.tick();
        }
    }
}
