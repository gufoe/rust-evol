use std::{
    path::{PathBuf},
};

use render::Render;
use server::Server;
use structopt::StructOpt;

mod actions;
mod pool;
mod genome;
mod input;
mod net;
mod render;
mod replicant;
mod server;
mod simulation;
mod world;
mod rng;

/// A fictional versioning CLI
#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]

struct Cli {
    #[structopt(short, long)]
    render: bool,
    #[structopt(short, long)]
    file: Option<PathBuf>,
}

fn main() {
    let args = Cli::from_args();

    let mut server: Server = if let Ok(content) = std::fs::read(&args.file.clone().unwrap_or("xxx".into())) {
        let mut server: Server = bincode::deserialize(&content).unwrap();
        server.sim.replicants.iter_mut().for_each(|x| x.net.update_nodes());
        server
    } else {
        Server::default()
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
