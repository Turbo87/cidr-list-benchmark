use bitstring_trees::set::RadixSet;
use cidr::{AnyIpCidr, IpCidr};
use divan::counter::ItemsCount;
use divan::Bencher;
use ip_network_table::IpNetworkTable;
use rand::random;
use std::net::IpAddr;

const CIDRS: &[&str] = &[
    // CloudFront IP addresses from http://d7uri8nf7uskq.cloudfront.net/tools/list-cloudfront-ips
    "3.10.17.128/25",
    "3.11.53.0/24",
    "3.35.130.128/25",
    "3.101.158.0/23",
    "3.128.93.0/24",
    "3.134.215.0/24",
    "3.231.2.0/25",
    "3.234.232.224/27",
    "3.236.48.0/23",
    "3.236.169.192/26",
    "13.32.0.0/15",
    "13.35.0.0/16",
    "13.48.32.0/24",
    "13.54.63.128/26",
    "13.59.250.0/26",
    "13.113.196.64/26",
    "13.113.203.0/24",
    "13.124.199.0/24",
    "13.210.67.128/26",
    "13.224.0.0/14",
    "13.228.69.0/24",
    "13.233.177.192/26",
    "13.249.0.0/16",
    "15.158.0.0/16",
    "15.188.184.0/24",
    "15.207.13.128/25",
    "15.207.213.128/25",
    "18.64.0.0/14",
    "18.154.0.0/15",
    "18.160.0.0/15",
    "18.164.0.0/15",
    "18.172.0.0/15",
    "18.192.142.0/23",
    "18.200.212.0/23",
    "18.216.170.128/25",
    "18.229.220.192/26",
    "18.238.0.0/15",
    "18.244.0.0/15",
    "34.195.252.0/24",
    "34.216.51.0/25",
    "34.223.12.224/27",
    "34.223.80.192/26",
    "34.226.14.0/24",
    "35.158.136.0/24",
    "35.162.63.192/26",
    "35.167.191.128/26",
    "36.103.232.0/25",
    "36.103.232.128/26",
    "44.227.178.0/24",
    "44.234.90.252/30",
    "44.234.108.128/25",
    "52.15.127.128/26",
    "52.46.0.0/18",
    "52.47.139.0/24",
    "52.52.191.128/26",
    "52.56.127.0/25",
    "52.57.254.0/24",
    "52.66.194.128/26",
    "52.78.247.128/26",
    "52.82.128.0/19",
    "52.84.0.0/15",
    "52.124.128.0/17",
    "52.199.127.192/26",
    "52.212.248.0/26",
    "52.220.191.0/26",
    "52.222.128.0/17",
    "54.182.0.0/16",
    "54.192.0.0/16",
    "54.230.0.0/17",
    "54.230.128.0/18",
    "54.230.200.0/21",
    "54.230.208.0/20",
    "54.230.224.0/19",
    "54.233.255.128/26",
    "54.239.128.0/18",
    "54.239.192.0/19",
    "54.240.128.0/18",
    "58.254.138.0/25",
    "58.254.138.128/26",
    "64.252.64.0/18",
    "64.252.128.0/18",
    "65.8.0.0/16",
    "65.9.0.0/17",
    "65.9.128.0/18",
    "70.132.0.0/18",
    "71.152.0.0/17",
    "99.79.169.0/24",
    "99.84.0.0/16",
    "99.86.0.0/16",
    "108.138.0.0/15",
    "108.156.0.0/14",
    "116.129.226.0/25",
    "116.129.226.128/26",
    "118.193.97.64/26",
    "118.193.97.128/25",
    "119.147.182.0/25",
    "119.147.182.128/26",
    "120.52.12.64/26",
    "120.52.22.96/27",
    "120.52.39.128/27",
    "120.52.153.192/26",
    "120.232.236.0/25",
    "120.232.236.128/26",
    "120.253.240.192/26",
    "120.253.241.160/27",
    "120.253.245.128/26",
    "120.253.245.192/27",
    "130.176.0.0/17",
    "130.176.128.0/18",
    "130.176.192.0/19",
    "130.176.224.0/20",
    "143.204.0.0/16",
    "144.220.0.0/16",
    "180.163.57.0/25",
    "180.163.57.128/26",
    "204.246.164.0/22",
    "204.246.168.0/22",
    "204.246.172.0/24",
    "204.246.173.0/24",
    "204.246.174.0/23",
    "204.246.176.0/20",
    "205.251.200.0/21",
    "205.251.208.0/20",
    "205.251.249.0/24",
    "205.251.250.0/23",
    "205.251.252.0/23",
    "205.251.254.0/24",
    "216.137.32.0/19",
    "223.71.11.0/27",
    "223.71.71.96/27",
    "223.71.71.128/25",
];

fn main() {
    divan::main();
}

#[divan::bench(min_time = 1)]
fn cidr_vec_iter(bencher: Bencher) {
    let cidrs: Vec<IpCidr> = CIDRS.iter().map(|s| s.parse().unwrap()).collect();

    bencher
        .counter(ItemsCount::new(1usize))
        .with_inputs(|| IpAddr::from([random(), random(), random(), random()]))
        .bench_values(|ip| {
            cidrs
                .iter()
                .any(|trusted_proxy| trusted_proxy.contains(&ip))
        })
}

#[divan::bench(min_time = 1)]
fn cidr_bitstring_tree(bencher: Bencher) {
    let mut cidrs = RadixSet::new();
    for s in CIDRS {
        cidrs.insert(s.parse::<AnyIpCidr>().unwrap());
    }

    bencher
        .counter(ItemsCount::new(1usize))
        .with_inputs(|| IpAddr::from([random(), random(), random(), random()]))
        .bench_values(|ip| {
            // TODO this seems wrong. how do you use this correctly?!
            cidrs
                .iter()
                .any(|trusted_proxy| trusted_proxy.contains(&ip))
        })
}

#[divan::bench(min_time = 1)]
fn ip_network_table(bencher: Bencher) {
    let mut cidrs = IpNetworkTable::new();
    for s in CIDRS {
        cidrs.insert(s.parse::<ip_network::IpNetwork>().unwrap(), ());
    }

    bencher
        .counter(ItemsCount::new(1usize))
        .with_inputs(|| IpAddr::from([random(), random(), random(), random()]))
        .bench_values(|ip| cidrs.matches(ip).next().is_some())
}

#[divan::bench(min_time = 1)]
fn ipnetwork_vec_iter(bencher: Bencher) {
    let cidrs: Vec<ipnetwork::IpNetwork> = CIDRS.iter().map(|s| s.parse().unwrap()).collect();

    bencher
        .counter(ItemsCount::new(1usize))
        .with_inputs(|| IpAddr::from([random(), random(), random(), random()]))
        .bench_values(|ip| cidrs.iter().any(|trusted_proxy| trusted_proxy.contains(ip)))
}

#[divan::bench(min_time = 1)]
fn ip_network_vec_iter(bencher: Bencher) {
    let cidrs: Vec<ip_network::IpNetwork> = CIDRS.iter().map(|s| s.parse().unwrap()).collect();

    bencher
        .counter(ItemsCount::new(1usize))
        .with_inputs(|| IpAddr::from([random(), random(), random(), random()]))
        .bench_values(|ip| cidrs.iter().any(|trusted_proxy| trusted_proxy.contains(ip)))
}
