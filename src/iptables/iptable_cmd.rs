use std::error::Error;
use std::io::Read;
use ssh2::Channel;
use std::cell::RefCell;
use crate::session::channel::get_channel;

//iptables 5表4链 定义常量 ，目前最常用到的是filter表和nat表
const T_FILTER: &'static str = "filter";
const T_NAT: &'static str = "nat";

const C_INPUT: &'static str = "INPUT";
const C_FORWARD: &'static str = "FORWARD";
const C_OUTPUT: &'static str = "OUTPUT";
const C_PREROUTING: &'static str = "PREROUTING";
const C_POSTROUTING: &'static str = "POSTROUTING";
const C_DOCKER: &'static str = "DOCKER";

const BUILTIN_CHAINS_FILTER: &[&str] = &[C_INPUT, C_FORWARD, C_OUTPUT, C_DOCKER];
const BUILTIN_CHAINS_NAT: &[&str] = &[C_PREROUTING, C_POSTROUTING, C_OUTPUT, C_DOCKER];


pub struct IpTablesCmd {
    pub  channel: RefCell<Channel>,
    /// Indicates if iptables has -C (--check) option
    pub has_check: bool,
    /// Indicates if iptables has -w (--wait) option
    pub has_wait: bool,
    /// Indicates if iptables will be run with -n (--numeric) option
    pub is_numeric: bool,
}

fn get_builtin_chains(table: &str) -> Result<&[&str], Box<dyn Error>> {
    match table {
        "filter" => Ok(BUILTIN_CHAINS_FILTER),
        //"mangle" => Ok(BUILTIN_CHAINS_MANGLE),
        "nat" => Ok(BUILTIN_CHAINS_NAT),
       // "raw" => Ok(BUILTIN_CHAINS_RAW),
        //"security" => Ok(BUILTIN_CHAINS_SECURITY),
        _ => Err(error_from_str("given table is not supported by iptables")),
    }
}

fn error_from_str(msg: &str) -> Box<dyn Error> {
    msg.into()
}



impl IpTablesCmd {
    pub fn get_policy(&self, table: &str, chain: &str) -> Result<String, Box<dyn Error>> {
        let builtin_chains = get_builtin_chains(table)?;
        if !builtin_chains.iter().as_slice().contains(&chain) {
            return Err(error_from_str(
                "given chain is not a default chain in the given table, can't get policy",
            ));
        }


        let cmd = match self.is_numeric {
            true => format!("iptables -t {} -L {} -n -v", table, chain),
            false => format!("iptables -t {} -L {} -v", table, chain),
        };
        self.channel.borrow_mut().exec(&cmd).unwrap();
        let mut iptables_l_n_cmd = String::new();
        self.channel.borrow_mut().read_to_string(&mut iptables_l_n_cmd).unwrap();
        return Ok(iptables_l_n_cmd);
    }

    pub fn new() -> IpTablesCmd {
        let m_channel = RefCell::new(get_channel());
        return IpTablesCmd {
            channel: m_channel,
            has_check: false,
            has_wait: false,
            is_numeric: false,
        };
    }
}
