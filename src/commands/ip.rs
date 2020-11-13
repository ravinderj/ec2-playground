use crate::ec2::Ec2IpAddress;
use rusoto_ec2::Filter;
use rusoto_ec2::{DescribeInstancesRequest, Ec2Client};
use rusoto_ec2::{DescribeInstancesResult, Ec2};

pub fn show_ips(client: Ec2Client, tag: Vec<String>) {
    let request = create_describe_instance_request(tag);
    let result = client.describe_instances(request).sync().unwrap();
    let ip_addresses = extract_ec2_ips(result);
    let json = serde_json::to_string_pretty(&ip_addresses).unwrap();

    println!("{}", json);
}

fn create_search_filters(tags: Vec<String>) -> Vec<Filter> {
    let mut filters = Vec::new();
    let mut values = Vec::with_capacity(tags.len());
    for tag in tags {
        values.push(String::from(format!("*{}*", tag)));
    }

    filters.push(Filter {
        name: Some("tag:Name".to_string()),
        values: Some(values),
    });

    filters
}

fn create_describe_instance_request(tags: Vec<String>) -> DescribeInstancesRequest {
    let filters = create_search_filters(tags);
    DescribeInstancesRequest {
        dry_run: Some(false),
        filters: Some(filters),
        max_results: Some(100),
        instance_ids: Some(Vec::new()),
        next_token: Some(String::from("")),
    }
}

fn extract_ec2_ips(describe_instance_result: DescribeInstancesResult) -> Vec<Ec2IpAddress> {
    let mut ip_addresses = Vec::new();
    if let Some(reservations) = describe_instance_result.reservations {
        for reservation in reservations {
            if let Some(instances) = reservation.instances {
                for instance in instances {
                    ip_addresses.push(Ec2IpAddress::from_instance(instance));
                }
            }
        }
    }
    ip_addresses
}
