use super::build::{self, HttpConfig};
use crate::{config::Config, proxy::Proxies};
use cidr::IpCidr;
use rand::Rng;
use rquest::Client;
use std::{
    net::IpAddr,
    sync::atomic::{AtomicUsize, Ordering},
};

pub enum Pool {
    Default(Client),
    Ifaces {
        load_factor: AtomicUsize,
        clients: Vec<Client>,
    },
    Proxy {
        load_factor: AtomicUsize,
        clients: Vec<Client>,
    },
    CIDR {
        load_factor: AtomicUsize,
        config: HttpConfig,
        cidr: Vec<IpCidr>,
    },
}

impl Pool {
    pub async fn new(conf: Config) -> Self {
        // split proxy
        let (proxies, ifaces, cidr): (Vec<_>, Vec<_>, Vec<_>) = conf.proxies.into_iter().fold(
            (vec![], vec![], vec![]),
            |(mut proxies, mut interfaces, mut cidr), proxy| {
                match proxy {
                    Proxies::Proxy(v) => proxies.push(v),
                    Proxies::Interface(v) => interfaces.push(v),
                    Proxies::CIDR(v) => cidr.push(v),
                }
                (proxies, interfaces, cidr)
            },
        );

        #[cfg(target_os = "linux")]
        for ip_cidr in cidr.iter() {
            {
                super::route::sysctl_ipv6_no_local_bind();
                super::route::sysctl_route_add_cidr(&ip_cidr).await;
            }
        }

        // Priority: cidr > proxies > ifaces
        match (!cidr.is_empty(), !proxies.is_empty(), !ifaces.is_empty()) {
            (true, _, _) => {
                let config = HttpConfig::builder()
                    .timeout(conf.timeout)
                    .connect_timeout(conf.connect_timeout)
                    .tcp_keepalive(conf.tcp_keepalive)
                    .build();

                Pool::CIDR {
                    config,
                    load_factor: AtomicUsize::new(0),
                    cidr,
                }
            }
            (false, true, _) => {
                let mut clients = vec![];

                for proxy_url in proxies {
                    let config = HttpConfig::builder()
                        .timeout(conf.timeout)
                        .connect_timeout(conf.connect_timeout)
                        .tcp_keepalive(conf.tcp_keepalive)
                        .proxy_url(proxy_url)
                        .build();

                    let client = build::build_client(config).await;
                    clients.push(client);
                }

                Pool::Proxy {
                    load_factor: AtomicUsize::new(0),
                    clients,
                }
            }
            (false, false, true) => {
                let mut clients = vec![];

                for iface in ifaces {
                    let config = HttpConfig::builder()
                        .timeout(conf.timeout)
                        .connect_timeout(conf.connect_timeout)
                        .tcp_keepalive(conf.tcp_keepalive)
                        .iface(iface)
                        .build();

                    let client = build::build_client(config).await;
                    clients.push(client);
                }

                Pool::Ifaces {
                    load_factor: AtomicUsize::new(0),
                    clients,
                }
            }
            _ => {
                let config = HttpConfig::builder()
                    .timeout(conf.timeout)
                    .connect_timeout(conf.connect_timeout)
                    .tcp_keepalive(conf.tcp_keepalive)
                    .build();

                Pool::Default(build::build_client(config).await)
            }
        }
    }

    #[inline]
    pub async fn load_client(&self) -> Client {
        match self {
            Pool::Default(client) => client.clone(),
            Pool::Ifaces {
                clients,
                load_factor,
            } => {
                let index = round_robin_factor(clients.len(), load_factor);
                clients[index].clone()
            }
            Pool::Proxy {
                clients,
                load_factor,
            } => {
                let index = round_robin_factor(clients.len(), load_factor);
                clients[index].clone()
            }
            Pool::CIDR {
                load_factor,
                config,
                cidr,
            } => {
                let index = round_robin_factor(cidr.len(), load_factor);
                let cidr = cidr[index];

                let addr = match cidr.first_address() {
                    IpAddr::V4(v4) => {
                        let v4 = u32::from(v4);
                        let prefix_len = cidr.network_length();
                        let rand: u32 = rand::thread_rng().gen();
                        let net_part = (v4 >> (32 - prefix_len)) << (32 - prefix_len);
                        let host_part = (rand << prefix_len) >> prefix_len;
                        Some(IpAddr::V4((net_part | host_part).into()))
                    }
                    IpAddr::V6(v6) => {
                        let ipv6 = u128::from(v6);
                        let prefix_len = cidr.network_length();
                        let rand: u128 = rand::thread_rng().gen();
                        let net_part = (ipv6 >> (128 - prefix_len)) << (128 - prefix_len);
                        let host_part = (rand << prefix_len) >> prefix_len;
                        Some(IpAddr::V6((net_part | host_part).into()))
                    }
                };

                let mut config = config.clone();
                config.set_iface(addr);
                build::build_client(config).await
            }
        }
    }
}

pub fn round_robin_factor(len: usize, counter: &AtomicUsize) -> usize {
    let mut old = counter.load(Ordering::Relaxed);
    let mut new;
    loop {
        new = (old + 1) % len;
        match counter.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
            Ok(_) => break,
            Err(x) => old = x,
        }
    }
    new
}
