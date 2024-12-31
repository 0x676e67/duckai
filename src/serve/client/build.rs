use super::dns;
use hickory_resolver::config::LookupIpStrategy;
use rand::seq::SliceRandom;
use rquest::{tls::Impersonate, Client, ClientBuilder, Proxy};
use std::{net::IpAddr, time::Duration};
use typed_builder::TypedBuilder;
use url::Url;

#[derive(TypedBuilder, Clone)]
pub struct HttpConfig {
    /// Timeout for each request.
    timeout: u64,

    /// Timeout for each connecting.
    connect_timeout: u64,

    /// TCP keepalive interval.
    tcp_keepalive: Option<u64>,

    // interface address
    #[builder(default, setter(into))]
    iface: Option<IpAddr>,

    // proxy
    #[builder(default, setter(into))]
    proxy_url: Option<Url>,
}

impl HttpConfig {
    pub fn set_iface(&mut self, iface: Option<IpAddr>) {
        self.iface = iface;
    }
}

/// Build a client
pub async fn build_client(config: HttpConfig) -> Client {
    init_builder(config)
        .await
        .build()
        .expect("Failed to build Api client")
}

/// Initialize a client builder
pub async fn init_builder(config: HttpConfig) -> ClientBuilder {
    let mut builder = Client::builder();

    // set proxy
    builder = set_proxy(builder, config.proxy_url);

    // disable keep alive
    builder = set_tcp_keepalive(builder, config.tcp_keepalive);

    // return lookup ip strategy
    let (builder, lookup_ip_strategy) =
        set_local_address_and_lookup_ip_strategy(builder, config.iface);

    // init dns resolver
    set_dns_resolver(builder, lookup_ip_strategy)
        .await
        .impersonate(random_impersonate())
        .cookie_store(true)
        .timeout(Duration::from_secs(config.timeout))
        .connect_timeout(Duration::from_secs(config.connect_timeout))
}

fn set_proxy(builder: ClientBuilder, proxy: Option<Url>) -> ClientBuilder {
    if let Some(proxy) = proxy {
        // If there is only one proxy, use it
        let proxy = Proxy::all(proxy).expect("Failed to create proxy");
        builder.proxy(proxy)
    } else {
        // If there is no proxy, use the system proxy
        builder
    }
}

fn set_tcp_keepalive(builder: ClientBuilder, tcp_keepalive: Option<u64>) -> rquest::ClientBuilder {
    match tcp_keepalive {
        Some(tcp_keepalive) => builder.tcp_keepalive(Duration::from_secs(tcp_keepalive)),
        None => builder.tcp_keepalive(None).pool_max_idle_per_host(0),
    }
}

fn set_local_address_and_lookup_ip_strategy(
    mut builder: rquest::ClientBuilder,
    preferred_addrs: Option<IpAddr>,
) -> (rquest::ClientBuilder, LookupIpStrategy) {
    let lookup_ip_strategy = match preferred_addrs {
        Some(ip_addr) => {
            builder = builder.local_address(ip_addr);
            if ip_addr.is_ipv4() {
                LookupIpStrategy::Ipv4Only
            } else {
                LookupIpStrategy::Ipv6Only
            }
        }
        None => LookupIpStrategy::Ipv4AndIpv6,
    };

    (builder, lookup_ip_strategy)
}

async fn set_dns_resolver(builder: ClientBuilder, ip_s: LookupIpStrategy) -> ClientBuilder {
    let trust_dns_resolver = dns::get_dns_resolver(ip_s).await;
    builder.dns_resolver(trust_dns_resolver)
}

fn random_impersonate() -> Impersonate {
    static VERSIONS: &'static [Impersonate] = &[
        Impersonate::Chrome100,
        Impersonate::Chrome101,
        Impersonate::Chrome104,
        Impersonate::Chrome105,
        Impersonate::Chrome106,
        Impersonate::Chrome107,
        Impersonate::Chrome108,
        Impersonate::Chrome109,
        Impersonate::Chrome114,
        Impersonate::Chrome116,
        Impersonate::Chrome117,
        Impersonate::Chrome118,
        Impersonate::Chrome119,
        Impersonate::Chrome120,
        Impersonate::Chrome123,
        Impersonate::Chrome124,
        Impersonate::Chrome126,
        Impersonate::Chrome127,
        Impersonate::Chrome128,
        Impersonate::Chrome129,
        Impersonate::Chrome130,
        Impersonate::Chrome131,
        Impersonate::SafariIos17_2,
        Impersonate::SafariIos17_4_1,
        Impersonate::SafariIos16_5,
        Impersonate::Safari15_3,
        Impersonate::Safari15_5,
        Impersonate::Safari15_6_1,
        Impersonate::Safari16,
        Impersonate::Safari16_5,
        Impersonate::Safari17_0,
        Impersonate::Safari17_2_1,
        Impersonate::Safari17_4_1,
        Impersonate::Safari17_5,
        Impersonate::Safari18,
        Impersonate::SafariIos18_1_1,
        Impersonate::Safari18_2,
        Impersonate::SafariIPad18,
        Impersonate::OkHttp3_9,
        Impersonate::OkHttp3_11,
        Impersonate::OkHttp3_13,
        Impersonate::OkHttp3_14,
        Impersonate::OkHttp4_9,
        Impersonate::OkHttp4_10,
        Impersonate::OkHttp5,
        Impersonate::Edge101,
        Impersonate::Edge122,
        Impersonate::Edge127,
        Impersonate::Edge131,
        Impersonate::Firefox109,
        Impersonate::Firefox117,
        Impersonate::Firefox133,
    ];

    *VERSIONS
        .choose(&mut rand::thread_rng())
        .unwrap_or(&Impersonate::default())
}
