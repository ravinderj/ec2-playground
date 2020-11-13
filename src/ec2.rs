use rusoto_ec2::Instance;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ec2IpAddress {
    public_ip: Option<String>,
    private_ip: Option<String>,
}

impl Ec2IpAddress {
    pub fn from_instance(instance: Instance) -> Self {
        Ec2IpAddress {
            public_ip: instance.public_ip_address,
            private_ip: instance.private_ip_address,
        }
    }
}
