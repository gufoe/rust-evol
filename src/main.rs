use std::{
    path::{PathBuf},
};

use render::Render;
use server::Server;
use structopt::StructOpt;

mod actions;
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
    file: PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let mut server: Server = if let Ok(content) = std::fs::read(&args.file) {
        let mut server: Server = bincode::deserialize(&content).unwrap();
        server.sim.replicants.iter_mut().for_each(|x| x.net.update_nodes());
        server
    } else {
        let mut server = Server::default();
        server.setup();
        server
    };

    server.auto_save = Some(args.file);

    if args.render {
        Render::new(server);
    } else {
        loop {
            server.tick();
        }
    }
}
