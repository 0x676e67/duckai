use cidr::IpCidr;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::net::IpAddr;
use url::Url;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Proxies {
    /// Upstream proxy, supports http, https, socks4, socks5, socks5h
    URL(Url),
    /// Bind to interface, supports ipv4, ipv6
    Iface(IpAddr),
    /// Bind to ipv6/ipv4 CIDR, ramdomly generate ipv4/ipv6 address
    CIDR(IpCidr),
}

impl Debug for Proxies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Proxies::URL(url) => write!(f, "{}", url),
            Proxies::Iface(ip_addr) => write!(f, "{}", ip_addr),
            Proxies::CIDR(cidr) => write!(f, "{}", cidr),
        }
    }
}

impl From<Url> for Proxies {
    fn from(url: Url) -> Self {
        Proxies::URL(url)
    }
}

impl From<IpAddr> for Proxies {
    fn from(ip_addr: IpAddr) -> Self {
        Proxies::Iface(ip_addr)
    }
}

impl From<IpCidr> for Proxies {
    fn from(cidr: IpCidr) -> Self {
        Proxies::CIDR(cidr)
    }
}