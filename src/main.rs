use ec2_bridge::commands::ip;
use rusoto_core::Region;
use rusoto_ec2::Ec2Client;

fn main() {
    let client = Ec2Client::new(Region::EuWest1);
    ip::show_ips(client, vec!["airflow".to_string(), "sonar".to_string()]);
}
