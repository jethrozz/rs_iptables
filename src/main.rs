use std::io::prelude::*;
use std::net::TcpStream;
use ssh2::{Channel,Session};


const HOST: &'static str = "127.0.0.1";
const PORT: u16 = 22;
const USER_NAME: &'static str = "root";
const PASSWORD: &'static str = "1234567";

//iptables 5表4链 定义常量 ，目前最常用到的是filter表和nat表
const T_FILTER: &'static str = "filter";
const T_NAT: &'static str = "nat";

const C_INPUT: &'static str = "INPUT";
const C_FORWARD: &'static str = "FORWARD";
const C_OUTPUT: &'static str = "OUTPUT";
const C_PREROUTING: &'static str = "PREROUTING";
const C_POSTROUTING: &'static str = "POSTROUTING";
const C_DOCKER_NETWORK: &'static str = "DOCKER";


fn main() {
    let tcp = TcpStream::connect((HOST, PORT)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);

    sess.handshake().unwrap();
    sess.userauth_password(USER_NAME, PASSWORD).unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.exec("iptables -L -n --line-numbers").unwrap();
    let mut iptables_l_n_cmd = String::new();
    channel.read_to_string(&mut iptables_l_n_cmd).unwrap();
    println!("{}", iptables_l_n_cmd);
    close_channel(&mut channel);
}


fn close_channel(channel: &mut Channel) {
    channel.close().unwrap();
    channel.wait_close().unwrap();
}