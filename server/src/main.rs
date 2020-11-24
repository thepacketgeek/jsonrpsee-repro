use std::net::{Ipv4Addr, SocketAddr};

use jsonrpsee::{raw::RawServer, transport::http::HttpTransportServer};
use rpc_lib::{Api, LearnedRoute};
use serde_json::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cli", rename_all = "kebab-case")]
/// CLI to interact with Server Example
struct Args {
    #[structopt(short, long, default_value = "127.0.0.1")]
    host: String,
    #[structopt(short, long, default_value = "8080")]
    port: u16,
    #[structopt(short, long)]
    // Used to repro issue, a limit < 32 will work fine
    limit: Option<usize>,
}

async fn serve(args: &Args) {
    let host: Ipv4Addr = args.host.parse().expect("Provide a valid IPv4 address");
    let socket: SocketAddr = (host, args.port).into();
    println!("Starting JSON-RPC server on {}...", socket);
    let mut server = {
        let transport_server = HttpTransportServer::bind(&socket).await.unwrap();
        RawServer::new(transport_server)
    };

    while let Ok(request) = Api::next_request(&mut server).await {
        println!("Waiting for next request...");
        match request {
            Api::ShowRoutes { respond } => {
                let mut response = get_example_data().unwrap();
                if let Some(limit) = args.limit {
                    response = response.into_iter().take(limit).collect();
                }
                let route_count = response.len();
                respond.ok(response);
                println!("Responded with {} routes", route_count);
            }
        }
    }
}

fn get_example_data() -> Result<Vec<LearnedRoute>> {
    let example_data = include_str!("../example_data.json");
    serde_json::from_str(example_data)
}

#[tokio::main]
async fn main() {
    let args = Args::from_args();
    serve(&args).await;
}
