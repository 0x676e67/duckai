use cidr::IpCidr;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Debug;
use std::net::IpAddr;
use url::Url;

#[derive(Serialize, Clone)]
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

impl<'de> Deserialize<'de> for Proxies {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let proxy = match (s.parse::<IpAddr>(), Url::parse(&s), s.parse::<IpCidr>()) {
            (Ok(ip_addr), _, _) => Proxies::from(ip_addr),
            (_, Ok(url), _) => Proxies::from(url),
            (_, _, Ok(cidr)) => Proxies::from(cidr),
            _ => return Err(serde::de::Error::custom("failed to parse proxies")),
        };

        Ok(proxy)
    }
}
