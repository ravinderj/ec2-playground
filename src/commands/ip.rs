use crate::ec2::Ec2IpAddress;
use rusoto_ec2::Ec2;
use rusoto_ec2::Filter;
use rusoto_ec2::{DescribeInstancesRequest, Ec2Client};

pub fn show_ips(client: Ec2Client) {
    let mut filters = Vec::new();
    let mut values = Vec::new();
    values.push(String::from("*airflow*"));

    filters.push(Filter {
        name: Some("tag:Name".to_string()),
        values: Some(values),
    });
    let request = DescribeInstancesRequest {
        dry_run: Some(false),
        filters: Some(filters),
        max_results: Some(100),
        instance_ids: Some(Vec::new()),
        next_token: Some(String::from("")),
    };
    let result = client.describe_instances(request).sync().unwrap();

    let mut ip_addresses = Vec::new();
    if let Some(reservations) = result.reservations {
        for reservation in reservations {
            if let Some(instances) = reservation.instances {
                for instance in instances {
                    ip_addresses.push(Ec2IpAddress::from_instance(instance));
                }
            }
        }
    }

    let json = serde_json::to_string_pretty(&ip_addresses).unwrap();

    println!("{}", json);
}
