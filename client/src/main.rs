use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use jsonrpsee::{raw::RawClient, transport::http::HttpTransportClient};
use rpc_lib::Api;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cli", rename_all = "kebab-case")]
/// CLI to interact with Server Example
struct Args {
    #[structopt(short, long, default_value = "127.0.0.1")]
    host: String,
    #[structopt(short, long, default_value = "8080")]
    port: u16,
}

async fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let mut client = {
        let base = format!("http://{}:{}", args.host, args.port);
        let transport_client = HttpTransportClient::new(&base);
        RawClient::new(transport_client)
    };
    let routes: Vec<_> = Api::show_routes(&mut client).await?;
    for route in routes {
        let mut lines: Vec<String> = Vec::with_capacity(16);
        let next_hop = match route.afi.as_str() {
            "IPv4" => route
                .next_hop
                .unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
            "IPv6" => route
                .next_hop
                .unwrap_or(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0))),
            _ => unreachable!("Unknown AFI"),
        };
        lines.push(format!(
            "{}/{} {} via {}",
            route.afi, route.safi, route.prefix, next_hop,
        ));
        println!("{}", lines.join("\n"));
    }
    Ok(())
}

fn main() {
    let args = Args::from_args();
    let result = async_std::task::block_on(async { run(args).await });
    if let Err(err) = result {
        eprintln!("{}", err.to_string());
    }
}
