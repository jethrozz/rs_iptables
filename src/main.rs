pub mod session;
pub mod iptables;

use session::get_channel;
use iptables::IpTables;

fn main() {
    let channel = get_channel();
    let iptables = IpTables::new(channel);

    let result = iptables.get_policy("filter", "INPUT").unwrap();
    print!("{}", result)
}