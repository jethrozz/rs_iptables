mod iptables;
mod session;
mod config;

use crate::iptables::iptable_cmd::IpTablesCmd;

fn main() {
    let ip_tables_cmd = IpTablesCmd::new();

    let result = ip_tables_cmd.get_policy("filter", "INPUT").unwrap();
    print!("{}", result)
}