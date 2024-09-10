mod iptables;
mod session;
mod config;

use iptables::IpTables;

fn main() {
    let mut iptables = IpTables::new();

    let result = iptables.get_policy("filter", "INPUT").unwrap();
    print!("{}", result)
}