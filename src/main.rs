use ec2_bridge::commands::ip;
use rusoto_core::Region;
use rusoto_ec2::Ec2Client;
use structopt::StructOpt;

fn main() {
    let cli = Cli::from_args();
    let client = Ec2Client::new(Region::EuWest1);
    ip::show_ips(client, cli.tags);

}

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short, long)]
    tags: Vec<String>
}